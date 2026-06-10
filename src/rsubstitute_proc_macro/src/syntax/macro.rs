use crate::syntax::path;
use proc_macro2::{Span, TokenStream};
use syn::*;

pub(crate) fn vec(tokens: TokenStream, span: Span) -> Macro {
    let result = Macro {
        path: path::new(["vec"], span),
        bang_token: Token![!](span),
        delimiter: MacroDelimiter::Bracket(token::Bracket(span)),
        tokens,
    };

    return result;
}
