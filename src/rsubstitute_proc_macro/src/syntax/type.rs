use crate::syntax;
use proc_macro2::Span;
use syn::*;

mod reference_anonymization;

pub mod path;

pub use reference_anonymization::*;
pub use reference_anonymization::*;

pub(crate) fn vec_of(span: Span, vec_t: Type) -> TypePath {
    wrap(span, "Vec", vec_t)
}

pub(crate) fn arc_of(span: Span, vec_t: Type) -> TypePath {
    wrap(span, "Arc", vec_t)
}

fn wrap(span: Span, wrapper: &'static str, wrapped: Type) -> TypePath {
    let result = TypePath {
        qself: None,
        path: syntax::path::new_generics(span, [wrapper], GenericArgument::Type(wrapped)),
    };

    return result;
}
