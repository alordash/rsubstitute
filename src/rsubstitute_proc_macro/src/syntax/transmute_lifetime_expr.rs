use crate::constants;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::token::Paren;
use syn::*;

pub(crate) fn create(ident: Ident) -> Expr {
    let expr = create_for_expr(path::create_expr(ident));
    return expr;
}

pub(crate) fn create_for_expr(expr: Expr) -> Expr {
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
