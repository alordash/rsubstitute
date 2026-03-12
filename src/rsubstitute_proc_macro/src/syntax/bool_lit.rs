use proc_macro2::Span;
use syn::*;

pub(crate) fn create(value: bool) -> Expr {
    let result = Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Bool(LitBool::new(value, Span::call_site())),
    });
    return result;
}
