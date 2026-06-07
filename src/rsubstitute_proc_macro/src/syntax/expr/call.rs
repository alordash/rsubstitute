use crate::syntax::expr;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(fn_name: Expr, args: [Expr; N], span: Span) -> ExprCall {
    let result = ExprCall {
        attrs: Vec::new(),
        func: Box::new(fn_name),
        paren_token: token::Paren(span),
        args: args.into_iter().collect(),
    };

    return result;
}
