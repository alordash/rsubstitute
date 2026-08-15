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

pub(crate) fn allow_unused(span: Span) -> Attribute {
    allow(span, "unused")
}

pub(crate) fn allow_nonstandard_style(span: Span) -> Attribute {
    allow(span, "nonstandard_style")
}

pub(crate) fn allow_unreachable_pub(span: Span) -> Attribute {
    allow(span, "unreachable_pub")
}

pub(crate) fn allow_private_interfaces(span: Span) -> Attribute {
    allow(span, "private_interfaces")
}

pub(crate) fn allow_private_bounds(span: Span) -> Attribute {
    allow(span, "private_bounds")
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
