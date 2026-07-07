use proc_macro2::Span;
use syn::*;

pub(crate) fn rsubstitute_self(span: Span) -> Ident {
    Ident::new("__rsa_self", span)
}
