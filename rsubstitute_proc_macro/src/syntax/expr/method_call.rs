use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(
    span: Span,
    receiver: Expr,
    method: Ident,
    args: [Expr; N],
) -> ExprMethodCall {
    let result = ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(receiver),
        dot_token: Token![.](span),
        method,
        turbofish: None,
        paren_token: token::Paren(span),
        args: args.into_iter().collect(),
    };

    return result;
}
