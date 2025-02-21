pub(crate) mod parsing;
pub(crate) mod printing;
use quote::ToTokens;
#[proc_macro]
pub fn client(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let openapi: printing::Api = syn::parse::<parsing::Api>(input).unwrap().into();
    openapi.into_token_stream().into()
}
