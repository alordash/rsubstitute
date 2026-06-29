use crate::syntax::path;
use proc_macro2::Span;
use quote::ToTokens;
use syn::*;

// TODO - verify that this is actually needed (remove it from generated code and see if there are any warnings)
pub(crate) fn allow_unused_variables(span: Span) -> Attribute {
    allow(span, "unused_variables")
}

pub(crate) fn allow_unused_imports(span: Span) -> Attribute {
    allow(span, "unused_imports")
}

pub(crate) fn allow_non_camel_case_types(span: Span) -> Attribute {
    allow(span, "non_camel_case_types")
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
