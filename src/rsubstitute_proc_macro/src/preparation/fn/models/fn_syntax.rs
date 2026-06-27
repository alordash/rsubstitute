use super::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct FnSyntax {
    pub attributes: Vec<Attribute>,
    pub source_signature: Box<Signature>,
    pub visibility: Visibility,
    pub merged_generics: Generics,
    pub fn_ident: Ident,
    pub is_default: bool,
    pub maybe_self_type: Option<Receiver>,
    pub arguments: Vec<Argument>,
    pub maybe_base_impl: Option<Box<Block>>,
    pub spans: Spans,
}

pub(crate) struct Spans {
    pub inputs: Span,
}
