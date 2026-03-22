use crate::syntax::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn create(func: Expr, arg: Expr) -> Expr {
    let result = Expr::Call(ExprCall {
        attrs: Vec::new(),
        func: Box::new(func),
        paren_token: Default::default(),
        args: [arg].into_iter().collect(),
    });
    return result;
}

pub(crate) fn create_with_args(func: Expr, args: Vec<Expr>) -> Expr {
    let result = Expr::Call(ExprCall {
        attrs: Vec::new(),
        func: Box::new(func),
        paren_token: Default::default(),
        args: args.into_iter().collect(),
    });
    return result;
}

pub(crate) fn create_without_args(func: Expr) -> Expr {
    let result = Expr::Call(ExprCall {
        attrs: Vec::new(),
        func: Box::new(func),
        paren_token: Default::default(),
        args: Punctuated::new(),
    });
    return result;
}

pub(crate) fn create_from_ident(func: Ident, args: Vec<Expr>) -> Expr {
    let result = Expr::Call(ExprCall {
        attrs: Vec::new(),
        func: Box::new(path::create_expr(func)),
        paren_token: Default::default(),
        args: args.into_iter().collect(),
    });
    return result;
}
