use crate::syntax::path;
use proc_macro2::Span;
use quote::ToTokens;
use syn::*;

pub(crate) fn inline(span: Span) -> Attribute {
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::Path(path::new(span, ["inline"])),
    };
    return result;
}

pub(crate) fn doc_hidden(span: Span) -> Attribute {
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::List(MetaList {
            path: path::new(span, ["doc"]),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: Ident::new("hidden", span).to_token_stream(),
        }),
    };
    return result;
}

pub(crate) fn allow_unused_variables(span: Span) -> Attribute {
    allow(span, "unused_variables")
}

pub(crate) fn allow_unused_imports(span: Span) -> Attribute {
    allow(span, "unused_imports")
}

pub(crate) fn allow_non_camel_case_types(span: Span) -> Attribute {
    allow(span, "non_camel_case_types")
}

pub(crate) fn allow_non_snake_case(span: Span) -> Attribute {
    allow(span, "non_snake_case")
}

pub(crate) fn allow_unreachable_pub(span: Span) -> Attribute {
    allow(span, "unreachable_pub")
}

fn allow(span: Span, allowed: &'static str) -> Attribute {
    let result = Attribute {
        pound_token: Token![#](span),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket(span),
        meta: Meta::List(MetaList {
            path: path::new(span, ["allow"]),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: Ident::new(allowed, span).to_token_stream(),
        }),
    };
    return result;
}
