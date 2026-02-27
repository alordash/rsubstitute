use crate::di::SERVICES;
use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait IStdMemTransmuteExprFactory {
    fn create(&self, ident: Ident) -> Expr;

    fn create_for_expr(&self, expr: Expr) -> Expr;
}

pub(crate) struct StdMemTransmuteExprFactory {
    pub path_factory: Arc<dyn IPathFactory>,
}

impl IStdMemTransmuteExprFactory for StdMemTransmuteExprFactory {
    fn create(&self, ident: Ident) -> Expr {
        let expr = self.create_for_expr(self.path_factory.create_expr(ident));
        return expr;
    }

    fn create_for_expr(&self, expr: Expr) -> Expr {
        let expr = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(Self::STD_MEM_TRANSMUTE_FUNC_PATH_EXPR.clone()),
            paren_token: Default::default(),
            args: [expr].into_iter().collect(),
        });
        return expr;
    }
}

impl StdMemTransmuteExprFactory {
    const STD_MEM_TRANSMUTE_FUNC_PATH_EXPR: LazyCell<Expr> = LazyCell::new(|| {
        let path_factory = &SERVICES.path_factory;
        let expr = path_factory.create_expr_from_parts(vec![
            format_ident!("core"),
            format_ident!("mem"),
            format_ident!("transmute"),
        ]);
        return expr;
    });
}
