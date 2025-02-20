mod api;
mod op;
pub(crate) use api::*;
pub(crate) use op::*;
use quote::ToTokens;
#[proc_macro]
pub fn client(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let openapi: Api = syn::parse(input).unwrap();
    openapi.into_token_stream().into()
}
