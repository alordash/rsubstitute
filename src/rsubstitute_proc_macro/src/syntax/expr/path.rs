use crate::syntax::path;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(span: Span, path_parts: [&str; N]) -> ExprPath {
    let result = ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: path::new(span, path_parts),
    };

    return result;
}

pub(crate) fn new_generics<const N: usize>(
    span: Span,
    path_parts: [&str; N],
    generic_argument: GenericArgument,
) -> ExprPath {
    let result = ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: path::new_generics(span, path_parts, [generic_argument]),
    };

    return result;
}
