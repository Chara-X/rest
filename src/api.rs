use super::*;
use syn::parse;
#[derive(Default)]
pub struct Api {
    pub title: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub ops: Vec<Op>,
}
impl parse::Parse for Api {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut api = Api::default();
        for stmt in syn::Block::parse(input)?.stmts {
            match stmt {
                syn::Stmt::Expr(syn::Expr::Struct(syn::ExprStruct { fields, .. }), ..) => {
                    for field in fields {
                        let member = match field.member {
                            syn::Member::Named(ident) => ident.to_string(),
                            _ => panic!(),
                        };
                        let expr = match field.expr {
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit),
                                ..
                            }) => lit.value(),
                            _ => panic!(),
                        };
                        match member.as_str() {
                            "title" => api.title = expr,
                            "version" => api.version = expr,
                            "description" => api.description = expr,
                            "url" => api.url = expr,
                            _ => panic!(),
                        }
                    }
                }
                syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall { args, .. }), ..) => {
                    let method = match args.get(0) {
                        Some(syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        })) => lit.value().to_lowercase(),
                        _ => panic!(),
                    };
                    let path = match args.get(1) {
                        Some(syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        })) => lit.value().to_lowercase(),
                        _ => panic!(),
                    };
                    let input = match args.get(2) {
                        Some(syn::Expr::Path(syn::ExprPath { path, .. })) => Some(path.clone()),
                        _ => None,
                    };
                    let output = match args.get(3) {
                        Some(syn::Expr::Path(syn::ExprPath { path, .. })) => Some(path.clone()),
                        _ => None,
                    };
                    api.ops.push(Op {
                        method,
                        path,
                        input,
                        output,
                    })
                }
                _ => panic!(),
            }
        }
        Ok(api)
    }
}
impl quote::ToTokens for Api {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let url = &self.url;
        let ops = &self.ops;
        let api = self.to_openapi_tokens();
        tokens.extend(quote::quote! {
            use std::path;
            use reqwest::blocking;
            use schemars::generate;
            pub struct Client {
                pub client: blocking::Client,
                pub addr: String,
            }
            impl Client {
                #api
                pub fn new(client: blocking::Client, addr: &str) -> Self {
                    Self {
                        client,
                        addr: format!("{}{}", addr, #url),
                    }
                }
                #(#ops)*
            }
        });
    }
}
impl Api {
    fn to_openapi_tokens(&self) -> proc_macro2::TokenStream {
        let title = &self.title;
        let version = &self.version;
        let description = &self.description;
        let url = &self.url;
        let ops =self.ops.iter().map(|op|{
            let method = syn::Ident::new(&op.method.to_lowercase(), proc_macro2::Span::call_site());
            let path= &op.path;
            let input =match &op.input{
                Some(path)=> quote::quote! {
                    request_body: Some(openapiv3::ReferenceOr::Item(openapiv3::RequestBody {
                        content: indexmap::indexmap! {
                            "application/json".to_string() => openapiv3::MediaType {
                                schema: Some(openapiv3::ReferenceOr::Item(serde_json::from_value(generator.root_schema_for::<#path>().clone().to_value()).unwrap())),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    })),
                },
                None=>quote::quote! {},
            };
            let output = match &op.output{
                Some(path)=>quote::quote! {
                    responses: openapiv3::Responses {
                        default: Some(openapiv3::ReferenceOr::Item(openapiv3::Response {
                            content: indexmap::indexmap! {
                                "application/json".to_string() => openapiv3::MediaType {
                                    schema: Some(openapiv3::ReferenceOr::Item(serde_json::from_value(generator.root_schema_for::<#path>().clone().to_value()).unwrap())),
                                    ..Default::default()
                                }
                            },
                            ..Default::default()
                        })),
                        ..Default::default()
                    },
                },
                None=>quote::quote! {},
            };
            quote::quote! {
                if let openapiv3::ReferenceOr::Item(path) = api.paths.paths.entry(#path.to_string()).or_insert(openapiv3::ReferenceOr::Item(openapiv3::PathItem::default()))
                {
                    path.#method = Some(openapiv3::Operation {
                        parameters: parameters(#path),
                        #input
                        #output
                        ..Default::default()
                    });
                }
            }
        });
        quote::quote! {
            pub fn openapi() -> openapiv3::OpenAPI {
                pub fn parameters(path: &str) -> Vec<openapiv3::ReferenceOr<openapiv3::Parameter>> {
                    path::Path::new(path)
                        .components()
                        .map(|x| x.as_os_str().to_str().unwrap())
                        .filter(|x| x.starts_with("{") && x.ends_with("}"))
                        .map(|x| &x[1..x.len() - 1])
                        .map(|x| {
                            openapiv3::ReferenceOr::Item(openapiv3::Parameter::Path {
                                parameter_data: openapiv3::ParameterData {
                                    name: x.to_string(),
                                    format: openapiv3::ParameterSchemaOrContent::Schema(
                                        openapiv3::ReferenceOr::Item(openapiv3::Schema {
                                            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                                openapiv3::StringType {
                                                    ..Default::default()
                                                },
                                            )),
                                            schema_data: openapiv3::SchemaData {
                                                ..Default::default()
                                            },
                                        }),
                                    ),
                                    required: true,
                                    example: None,
                                    extensions: Default::default(),
                                    examples: Default::default(),
                                    explode: None,
                                    description: None,
                                    deprecated: None,
                                },
                                style: openapiv3::PathStyle::Simple,
                            })
                        })
                        .collect()
                }
                let mut api = openapiv3::OpenAPI {
                    openapi: "3.0.3".to_string(),
                    info: openapiv3::Info {
                        title: #title.to_string(),
                        version: #version.to_string(),
                        description: Some(#description.to_string()),
                        ..Default::default()
                    },
                    servers: vec![openapiv3::Server {
                        url: #url.to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let mut settings = generate::SchemaSettings::openapi3();
                settings.inline_subschemas = true;
                let generator = &mut schemars::SchemaGenerator::new(settings);
                #(#ops)*
                api
            }
        }
    }
}
