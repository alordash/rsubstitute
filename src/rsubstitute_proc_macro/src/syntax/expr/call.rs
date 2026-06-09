use crate::syntax::expr;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(func: Expr, args: [Expr; N], span: Span) -> ExprCall {
    let result = ExprCall {
        attrs: Vec::new(),
        func: Box::new(func),
        paren_token: token::Paren(span),
        args: args.into_iter().collect(),
    };

    return result;
}
