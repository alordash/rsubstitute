use crate::di::SERVICES;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait ICoreMemTransmuteExprFactory {
    fn create(&self, ident: Ident) -> Expr;

    fn create_for_expr(&self, expr: Expr) -> Expr;
}

pub(crate) struct CoreMemTransmuteExprFactory {
    pub path_factory: Arc<dyn IPathFactory>,
    pub expr_call_factory: Arc<dyn IExprCallFactory>,
}

impl ICoreMemTransmuteExprFactory for CoreMemTransmuteExprFactory {
    fn create(&self, ident: Ident) -> Expr {
        let expr = self.create_for_expr(self.path_factory.create_expr(ident));
        return expr;
    }

    fn create_for_expr(&self, expr: Expr) -> Expr {
        let result = self
            .expr_call_factory
            .create(Self::CORE_MEM_TRANSMUTE_FUNC_PATH_EXPR.clone(), expr);
        return result;
    }
}

impl CoreMemTransmuteExprFactory {
    const CORE_MEM_TRANSMUTE_FUNC_PATH_EXPR: LazyCell<Expr> = LazyCell::new(|| {
        let path_factory = &SERVICES.path_factory;
        let expr = path_factory.create_expr_from_parts(vec![
            format_ident!("core"),
            format_ident!("mem"),
            format_ident!("transmute"),
        ]);
        return expr;
    });
}
