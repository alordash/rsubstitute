use crate::di::SERVICES;
use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait IStdMemTransmuteExprFactory {
    fn create(&self, ident: Ident) -> Expr;
}

pub(crate) struct StdMemTransmuteExprFactory {
    pub path_factory: Arc<dyn IPathFactory>,
}

impl IStdMemTransmuteExprFactory for StdMemTransmuteExprFactory {
    fn create(&self, ident: Ident) -> Expr {
        let expr = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(Self::STD_MEM_TRANSMUTE_FUNC_PATH_EXPR.clone()),
            paren_token: Default::default(),
            args: [Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: self.path_factory.create(ident),
            })]
            .into_iter()
            .collect(),
        });
        return expr;
    }
}

impl StdMemTransmuteExprFactory {
    const STD_MEM_TRANSMUTE_FUNC_PATH_EXPR: LazyCell<Expr> = LazyCell::new(|| {
        let path_factory = &SERVICES.path_factory;
        let path = path_factory.create_from_parts(&[
            format_ident!("std"),
            format_ident!("mem"),
            format_ident!("transmute"),
        ]);
        let expr = Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path,
        });
        return expr;
    });
}
