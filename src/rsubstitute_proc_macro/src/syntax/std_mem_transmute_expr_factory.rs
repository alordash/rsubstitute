use crate::syntax::IExprMethodCallFactory;
use proc_macro2::Ident;
use quote::format_ident;
use std::rc::Rc;
use syn::Expr;

pub trait IStdMemTransmuteExprFactory {
    fn create(&self, ident: Ident) -> Expr;
}

pub(crate) struct StdMemTransmuteExprFactory {
    pub expr_method_call_factory: Rc<dyn IExprMethodCallFactory>,
}

impl IStdMemTransmuteExprFactory for StdMemTransmuteExprFactory {
    fn create(&self, ident: Ident) -> Expr {
        // TODO - it joins with '.', but '::' is needed
        let call = self.expr_method_call_factory.create(
            &[format_ident!("std"), format_ident!("mem")],
            format_ident!("transmute"),
            &[ident],
        );
        let expr = Expr::MethodCall(call);
        return expr;
    }
}
