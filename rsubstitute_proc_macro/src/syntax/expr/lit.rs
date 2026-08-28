use proc_macro2::Span;
use syn::*;

pub(crate) fn string(span: Span, string: &str) -> Expr {
    Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Str(LitStr::new(string, span)),
    })
}