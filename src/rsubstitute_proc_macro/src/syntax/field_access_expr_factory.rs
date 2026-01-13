use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use std::sync::Arc;
use syn::{Expr, ExprField, ExprPath, Member};

pub trait IFieldAccessExprFactory {
    fn create(&self, members_idents: Vec<Ident>) -> Expr;

    fn create_with_base_expr(&self, base_expr: Expr, member_idents: Vec<Ident>) -> Expr;
}

pub struct FieldAccessExprFactory {
    pub(crate) path_factory: Arc<dyn IPathFactory>,
}

impl IFieldAccessExprFactory for FieldAccessExprFactory {
    fn create(&self, mut members_idents: Vec<Ident>) -> Expr {
        let first_ident = members_idents.remove(0);
        let base_expr = Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: self.path_factory.create(first_ident.clone()),
        });
        let result = self.create_with_base_expr(base_expr, members_idents);
        return result;
    }

    fn create_with_base_expr(&self, base_expr: Expr, members_idents: Vec<Ident>) -> Expr {
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
}
