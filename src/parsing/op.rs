#[derive(Default)]
pub struct Op {
    pub summary: String,
    pub method: String,
    pub path: String,
    pub input: Option<syn::Path>,
    pub output: Option<syn::Path>,
}
