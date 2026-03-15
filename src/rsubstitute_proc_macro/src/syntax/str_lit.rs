use proc_macro2::{Ident, Span};
use syn::*;

pub(crate) fn create(string: &str) -> Expr {
    let result = Expr::Lit(ExprLit {
        attrs:Vec::new(),
        lit: Lit::Str(LitStr::new(string, Span::call_site()))
    });
    return result;
}

pub(crate) fn create_from_ident(ident: &Ident) -> Expr {
    create(&ident.to_string())
}