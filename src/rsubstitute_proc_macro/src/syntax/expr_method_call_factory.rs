use crate::syntax::{IFieldAccessExprFactory, IPathFactory};
use proc_macro2::Ident;
use std::sync::Arc;
use syn::*;

pub trait IExprMethodCallFactory {
    fn create(&self, members_idents: Vec<Ident>, method: Ident, args: Vec<Ident>)
    -> ExprMethodCall;

    fn create_with_expr_args(
        &self,
        members_idents: Vec<Ident>,
        method: Ident,
        args: Vec<Expr>,
    ) -> ExprMethodCall;

    fn create_with_base_receiver(
        &self,
        receiver: Expr,
        members_idents: Vec<Ident>,
        method: Ident,
        args: Vec<Ident>,
    ) -> ExprMethodCall;

    fn create_with_base_receiver_and_expr_args(
        &self,
        receiver: Expr,
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
        let result = self.create_with_expr_args(members_idents, method, self.convert_args(args));
        return result;
    }

    fn create_with_expr_args(
        &self,
        members_idents: Vec<Ident>,
        method: Ident,
        args: Vec<Expr>,
    ) -> ExprMethodCall {
        let receiver = self.field_access_expr_factory.create(members_idents);
        let expr_method_call = self.create(receiver, method, args);
        return expr_method_call;
    }

    fn create_with_base_receiver(
        &self,
        receiver: Expr,
        members_idents: Vec<Ident>,
        method: Ident,
        args: Vec<Ident>,
    ) -> ExprMethodCall {
        let expr_method_call = self.create_with_base_receiver_and_expr_args(
            receiver,
            members_idents,
            method,
            self.convert_args(args),
        );
        return expr_method_call;
    }

    fn create_with_base_receiver_and_expr_args(
        &self,
        receiver: Expr,
        members_idents: Vec<Ident>,
        method: Ident,
        args: Vec<Expr>,
    ) -> ExprMethodCall {
        let actual_receiver = self
            .field_access_expr_factory
            .create_with_base_expr(receiver, members_idents);
        let expr_method_call = self.create(actual_receiver, method, args);
        return expr_method_call;
    }
}

impl ExprMethodCallFactory {
    fn convert_args(&self, args: Vec<Ident>) -> Vec<Expr> {
        let expr_args = args
            .into_iter()
            .map(|arg| self.path_factory.create_expr(arg.clone()))
            .collect();
        return expr_args;
    }

    fn create(&self, receiver: Expr, method: Ident, args: Vec<Expr>) -> ExprMethodCall {
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
