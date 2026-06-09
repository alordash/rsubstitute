use crate::syntax::path;
use proc_macro2::Span;
use quote::ToTokens;
use syn::*;

pub(crate) fn allow_unused_variables(span: Span) -> Attribute {
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::List(MetaList {
            path: path::new(["allow"], span),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: Ident::new("unused_variables", span).to_token_stream(),
        }),
    };

    return result;
}
