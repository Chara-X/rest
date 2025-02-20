use super::*;
pub struct OpenApi {
    pub title: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub operations: Vec<Operation>,
}
impl quote::ToTokens for OpenApi {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let title = &self.title;
        let version = &self.version;
        let description = &self.description;
        let url = &self.url;
        let operations = &self.operations;
        tokens.extend(quote::quote! {
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
                                    description: None,
                                    explode: None,
                                    deprecated: None,
                                    example: None,
                                    examples: Default::default(),
                                    extensions: Default::default(),
                                },
                                style: openapiv3::PathStyle::Simple,
                            })
                        })
                        .collect()
                }
                let mut openapi = openapiv3::OpenAPI {
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
                #(#operations)*
                openapi
            }
        });
    }
}
