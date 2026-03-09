use proc_macro2::Span;
use syn::*;

pub trait IBoolLitFactory {
    fn create(&self, value: bool) -> Expr;
}

pub(crate) struct BoolLitFactory;

impl IBoolLitFactory for BoolLitFactory {
    fn create(&self, value: bool) -> Expr {
        let result = Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Bool(LitBool::new(
                value,
                Span::call_site(),
            )),
        });
        return result;
    }
}