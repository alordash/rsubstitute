use crate::syntax::path;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn new(expr: Expr, target_type: Type) -> Expr {
    let span = expr.span();
    let args: Punctuated<Expr, Token![,]> = [expr, Expr::Verbatim(target_type.to_token_stream())]
        .into_iter()
        .collect();
    let result = Expr::Macro(ExprMacro {
        attrs: Vec::new(),
        mac: Macro {
            path: path::new(span, ["transmute_lifetime"]),
            bang_token: Token![!](span),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: args.to_token_stream(),
        },
    });

    return result;
}
