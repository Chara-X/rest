use super::*;
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
