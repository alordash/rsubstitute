use crate::syntax::path;
use proc_macro2::{Span, TokenStream};
use syn::*;

pub(crate) fn vec(span: Span, tokens: TokenStream) -> Macro {
    let result = Macro {
        path: path::new(span, ["vec"]),
        bang_token: Token![!](span),
        delimiter: MacroDelimiter::Bracket(token::Bracket(span)),
        tokens,
    };

    return result;
}
