use crate::constants;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::ToTokens;
use std::sync::Arc;
use syn::token::Paren;
use syn::*;

pub(crate) trait ITransmuteLifetimeExprFactory {
    fn create(&self, ident: Ident) -> Expr;

    fn create_for_expr(&self, expr: Expr) -> Expr;
}

pub(crate) struct TransmuteLifetimeExprFactory {
    pub path_factory: Arc<dyn IPathFactory>,
}

impl ITransmuteLifetimeExprFactory for TransmuteLifetimeExprFactory {
    fn create(&self, ident: Ident) -> Expr {
        let expr = self.create_for_expr(self.path_factory.create_expr(ident));
        return expr;
    }

    fn create_for_expr(&self, expr: Expr) -> Expr {
        let result = Expr::Macro(ExprMacro {
            attrs: Vec::new(),
            mac: Macro {
                path: constants::TRANSMUTE_LIFETIME_MACRO_PATH.clone(),
                bang_token: Default::default(),
                delimiter: MacroDelimiter::Paren(Paren::default()),
                tokens: expr.to_token_stream(),
            },
        });

        return result;
    }
}
