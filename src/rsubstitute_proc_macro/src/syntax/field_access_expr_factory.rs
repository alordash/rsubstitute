use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use std::sync::Arc;
use syn::{Expr, ExprField, ExprPath, Member};

pub trait IFieldAccessExprFactory {
    fn create(&self, members_idents: &[Ident]) -> Expr;
}

pub struct FieldAccessExprFactory {
    pub(crate) path_factory: Arc<dyn IPathFactory>,
}

impl IFieldAccessExprFactory for FieldAccessExprFactory {
    fn create(&self, members_idents: &[Ident]) -> Expr {
        let base_expr = members_idents
            .first()
            .map(|first_ident| {
                Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: self.path_factory.create(first_ident.clone()),
                })
            })
            .expect("`idents` should contain at least one ident.");
        let receiver = members_idents
            .iter()
            .skip(1)
            .cloned()
            .fold(base_expr, |acc, x| {
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
