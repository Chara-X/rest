use std::path;
#[derive(Default)]
pub struct Op {
    pub method: String,
    pub path: String,
    pub input: Option<syn::Path>,
    pub output: Option<syn::Path>,
}
impl quote::ToTokens for Op {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let operation = syn::Ident::new(
            &format!("{}{}", self.method, self.path)
                .replace("/", "_")
                .replace("-", "_")
                .replace("{", "")
                .replace("}", ""),
            proc_macro2::Span::call_site(),
        );
        let method = syn::Ident::new(&self.method, proc_macro2::Span::call_site());
        let url = format!("{}{}", "{}", self.path);
        let parameters = path::Path::new(&self.path)
            .components()
            .map(|x| x.as_os_str().to_str().unwrap())
            .filter(|x| x.starts_with("{") && x.ends_with("}"))
            .map(|x| syn::Ident::new(&x[1..x.len() - 1], proc_macro2::Span::call_site()))
            .map(|x| quote::quote! {, #x: &str});
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
            pub fn #operation(&self #(#parameters)* #input) -> reqwest::Result<#output> {
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
