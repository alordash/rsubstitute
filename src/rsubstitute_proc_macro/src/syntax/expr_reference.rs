use syn::*;

pub(crate) fn create(base_expr: Expr) -> Expr {
    let expr_ref = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Default::default(),
        mutability: None,
        expr: Box::new(base_expr),
    });
    return expr_ref;
}
