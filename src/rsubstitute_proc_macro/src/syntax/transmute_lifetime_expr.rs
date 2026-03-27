use crate::constants;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::punctuated::Punctuated;
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

pub(crate) fn create_for_arg(field_ident: Ident, actual_source_type: Type) -> Expr {
    let result = create_for_expr(Expr::Verbatim(
        [
            reference::create_expr(field_access_expr::create(vec![
                constants::SELF_IDENT.clone(),
                field_ident.clone(),
            ])),
            reference::create_expr(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: arg_type::create(actual_source_type.clone()).path,
            })),
        ]
        .into_iter()
        .collect::<Punctuated<_, Token![,]>>()
        .to_token_stream(),
    ));
    return result;
}
