pub struct Op {
    pub name: syn::Ident,
    pub method: syn::Ident,
    pub parameters: Vec<syn::Ident>,
    pub path: String,
    pub input: Option<syn::Path>,
    pub output: Option<syn::Path>,
}
impl quote::ToTokens for Op {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let method = &self.method;
        let url = format!("{}{}", "{}", self.path);
        let parameters = &self.parameters;
        let input = match &self.input {
            Some(input) => quote::quote! {, body: &#input},
            None => quote::quote! {},
        };
        let output = match &self.output {
            Some(output) => quote::quote! {#output},
            None => quote::quote! {blocking::Response},
        };
        let json = match &self.input {
            Some(_) => quote::quote! {.json(body)},
            None => quote::quote! {},
        };
        let ret = match &self.output {
            Some(_) => quote::quote! {res.json()},
            None => quote::quote! {Ok(res)},
        };
        tokens.extend(quote::quote! {
            pub fn #name(&self #(, #parameters: &str)* #input) -> reqwest::Result<#output> {
                let res = self
                .client
                .#method(format!(#url, self.addr))
                #json
                .send()?;
                if let Err(err) = res.error_for_status_ref() {
                    eprintln!("{}", res.text().unwrap());
                    return Err(err);
                }
                #ret
            }
        });
    }
}
