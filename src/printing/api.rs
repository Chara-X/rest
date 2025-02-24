use super::*;
use crate::parsing;
use std::path;
pub struct Api {
    pub url: String,
    pub ops: Vec<Op>,
    pub openapi: OpenApi,
}
impl quote::ToTokens for Api {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let url = &self.url;
        let ops = &self.ops;
        let openapi = &self.openapi;
        tokens.extend(quote::quote! {
            use std::path;
            use reqwest::blocking;
            use schemars::generate;
            pub struct Client {
                pub client: blocking::Client,
                pub addr: String,
            }
            impl Client {
                #openapi
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
impl From<parsing::Api> for Api {
    fn from(value: parsing::Api) -> Self {
        Api {
            url: value.url.clone(),
            ops: value
                .ops
                .iter()
                .map(|op| Op {
                    name: syn::Ident::new(
                        &format!("{}{}", op.method, op.path)
                            .replace("/", "_")
                            .replace("-", "_")
                            .replace("{", "")
                            .replace("}", ""),
                        proc_macro2::Span::call_site(),
                    ),
                    method: syn::Ident::new(&op.method, proc_macro2::Span::call_site()),
                    path: op.path.clone(),
                    parameters: path::Path::new(&op.path)
                        .components()
                        .map(|x| x.as_os_str().to_str().unwrap())
                        .filter(|x| x.starts_with("{") && x.ends_with("}"))
                        .map(|x| {
                            syn::Ident::new(&x[1..x.len() - 1], proc_macro2::Span::call_site())
                        })
                        .collect(),
                    input: op.input.clone(),
                    output: op.output.clone(),
                })
                .collect(),
            openapi: OpenApi {
                title: value.title,
                version: value.version,
                description: value.description,
                url: value.url,
                operations: value
                    .ops
                    .iter()
                    .map(|op| Operation {
                        description: op.description.clone(),
                        method: syn::Ident::new(&op.method, proc_macro2::Span::call_site()),
                        path: op.path.clone(),
                        input: op.input.clone(),
                        output: op.output.clone(),
                    })
                    .collect(),
            },
        }
    }
}
