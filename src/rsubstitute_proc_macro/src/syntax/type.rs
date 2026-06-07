use crate::syntax;
use proc_macro2::Span;
use syn::*;

pub mod path;

pub(crate) fn vec_of(vec_t: Type, span: Span) -> TypePath {
    let result = TypePath {
        qself: None,
        path: syntax::path::new_generics(["Vec"], GenericArgument::Type(vec_t), span),
    };

    return result;
}
