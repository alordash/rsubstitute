use crate::syntax;
use proc_macro2::Span;
use syn::*;

pub mod path;

pub(crate) fn vec_of(span: Span, vec_t: Type) -> TypePath {
    let result = TypePath {
        qself: None,
        path: syntax::path::new_generics(span, ["Vec"], GenericArgument::Type(vec_t)),
    };

    return result;
}
