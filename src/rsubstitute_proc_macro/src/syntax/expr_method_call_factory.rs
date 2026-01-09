use crate::syntax::{IFieldAccessExprFactory, IPathFactory};
use proc_macro2::Ident;
use std::sync::Arc;
use syn::{Expr, ExprMethodCall, ExprPath};

pub trait IExprMethodCallFactory {
    fn create(&self, members_idents: Vec<Ident>, method: Ident, args: Vec<Ident>)
    -> ExprMethodCall;

    fn create_with_expr_args(
        &self,
        members_idents: Vec<Ident>,
        method: Ident,
        args: Vec<Expr>,
    ) -> ExprMethodCall;
}

pub struct ExprMethodCallFactory {
    pub(crate) path_factory: Arc<dyn IPathFactory>,
    pub(crate) field_access_expr_factory: Arc<dyn IFieldAccessExprFactory>,
}

impl IExprMethodCallFactory for ExprMethodCallFactory {
    fn create(
        &self,
        members_idents: Vec<Ident>,
        method: Ident,
        args: Vec<Ident>,
    ) -> ExprMethodCall {
        let result = self.create_with_expr_args(
            members_idents,
            method,
            args.into_iter()
                .map(|arg| {
                    Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: self.path_factory.create(arg.clone()),
                    })
                })
                .collect(),
        );
        return result;
    }

    fn create_with_expr_args(
        &self,
        members_idents: Vec<Ident>,
        method: Ident,
        args: Vec<Expr>,
    ) -> ExprMethodCall {
        let receiver = self.field_access_expr_factory.create(members_idents);
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
}
