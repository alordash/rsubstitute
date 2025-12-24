use crate::syntax::{IFieldAccessExprFactory, IPathFactory};
use proc_macro2::Ident;
use std::rc::Rc;
use syn::{Expr, ExprMethodCall, ExprPath};

pub trait IExprMethodCallFactory {
    fn create(
        &self,
        members_idents: &[Ident],
        method_ident: Ident,
        args: &[Ident],
    ) -> ExprMethodCall;
}

pub struct ExprMethodCallFactory {
    pub(crate) path_factory: Rc<dyn IPathFactory>,
    pub(crate) field_access_expr_factory: Rc<dyn IFieldAccessExprFactory>,
}

impl IExprMethodCallFactory for ExprMethodCallFactory {
    fn create(&self, members_idents: &[Ident], method: Ident, args: &[Ident]) -> ExprMethodCall {
        let receiver = self.field_access_expr_factory.create(members_idents);
        let expr_method_call = ExprMethodCall {
            attrs: Vec::new(),
            receiver: Box::new(receiver),
            dot_token: Default::default(),
            method,
            turbofish: None,
            paren_token: Default::default(),
            args: args
                .iter()
                .map(|arg| {
                    Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: self.path_factory.create(arg.clone()),
                    })
                })
                .collect(),
        };
        return expr_method_call;
    }
}
