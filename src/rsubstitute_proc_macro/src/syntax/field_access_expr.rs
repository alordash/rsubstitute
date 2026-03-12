use crate::syntax::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) fn create(mut members_idents: Vec<Ident>) -> Expr {
    let first_ident = members_idents.remove(0);
    let base_expr = path::create_expr(first_ident.clone());
    let result = create_with_base_expr(base_expr, members_idents);
    return result;
}

pub(crate) fn create_with_base_expr(base_expr: Expr, members_idents: Vec<Ident>) -> Expr {
    let receiver = members_idents.into_iter().fold(base_expr, |acc, x| {
        Expr::Field(ExprField {
            attrs: Vec::new(),
            base: Box::new(acc),
            dot_token: Default::default(),
            member: Member::Named(x),
        })
    });
    return receiver;
}
