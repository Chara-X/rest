pub struct Operation {
    pub summary: String,
    pub method: syn::Ident,
    pub path: String,
    pub input: Option<syn::Path>,
    pub output: Option<syn::Path>,
}
impl quote::ToTokens for Operation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let summary = &self.summary;
        let method = &self.method;
        let path = &self.path;
        let input = match &self.input {
            Some(path) => quote::quote! {
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
            None => quote::quote! {},
        };
        let output = match &self.output {
            Some(path) => quote::quote! {
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
            None => quote::quote! {},
        };
        tokens.extend(quote::quote! {
            if let openapiv3::ReferenceOr::Item(path) = openapi.paths.paths.entry(#path.to_string()).or_insert(openapiv3::ReferenceOr::Item(openapiv3::PathItem::default()))
            {
                path.#method = Some(openapiv3::Operation {
                    summary: Some(#summary.to_string()),
                    parameters: parameters(#path),
                    #input
                    #output
                    ..Default::default()
                });
            }
        });
    }
}
