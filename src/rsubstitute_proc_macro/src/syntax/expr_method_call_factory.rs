use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use std::rc::Rc;
use syn::{Expr, ExprField, ExprMethodCall, ExprPath, Member};

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
}

impl IExprMethodCallFactory for ExprMethodCallFactory {
    fn create(&self, members_idents: &[Ident], method: Ident, args: &[Ident]) -> ExprMethodCall {
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
