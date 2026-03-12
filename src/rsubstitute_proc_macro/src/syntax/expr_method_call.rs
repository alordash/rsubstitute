use crate::syntax::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) fn create(
    members_idents: Vec<Ident>,
    method: Ident,
    args: Vec<Ident>,
) -> ExprMethodCall {
    let result = create_with_expr_args(members_idents, method, convert_args(args));
    return result;
}

pub(crate) fn create_with_expr_args(
    members_idents: Vec<Ident>,
    method: Ident,
    args: Vec<Expr>,
) -> ExprMethodCall {
    let receiver = field_access_expr::create(members_idents);
    let expr_method_call = create_core(receiver, method, args);
    return expr_method_call;
}

pub(crate) fn create_with_base_receiver(
    receiver: Expr,
    members_idents: Vec<Ident>,
    method: Ident,
    args: Vec<Ident>,
) -> ExprMethodCall {
    let expr_method_call = create_with_base_receiver_and_expr_args(
        receiver,
        members_idents,
        method,
        convert_args(args),
    );
    return expr_method_call;
}

pub(crate) fn create_with_base_receiver_and_expr_args(
    receiver: Expr,
    members_idents: Vec<Ident>,
    method: Ident,
    args: Vec<Expr>,
) -> ExprMethodCall {
    let actual_receiver = field_access_expr::create_with_base_expr(receiver, members_idents);
    let expr_method_call = create_core(actual_receiver, method, args);
    return expr_method_call;
}

fn convert_args(args: Vec<Ident>) -> Vec<Expr> {
    let expr_args = args
        .into_iter()
        .map(|arg| path::create_expr(arg.clone()))
        .collect();
    return expr_args;
}

fn create_core(receiver: Expr, method: Ident, args: Vec<Expr>) -> ExprMethodCall {
    let expr_method_call = ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(receiver),
        dot_token: Default::default(),
        method,
        turbofish: None,
        paren_token: Default::default(),
        args: args.into_iter().collect(),
    };
    return expr_method_call;
}
