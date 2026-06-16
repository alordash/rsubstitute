use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span, value: bool) -> GenericArgument {
    let result = GenericArgument::Const(Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Bool(LitBool::new(value, span)),
    }));

    return result;
}
