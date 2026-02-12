use syn::*;

pub trait IExprReferenceFactory {
    fn create(&self, base_expr: Expr) -> Expr;
}

pub(crate) struct ExprReferenceFactory;

impl IExprReferenceFactory for ExprReferenceFactory {
    fn create(&self, base_expr: Expr) -> Expr {
        let expr_ref = Expr::Reference(ExprReference {
            attrs: Vec::new(),
            and_token: Default::default(),
            mutability: None,
            expr: Box::new(base_expr),
        });
        return expr_ref;
    }
}
