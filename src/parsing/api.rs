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
                        })) => lit.value(),
                        _ => panic!(),
                    };
                    let path = match args.get(1) {
                        Some(syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        })) => lit.value(),
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
