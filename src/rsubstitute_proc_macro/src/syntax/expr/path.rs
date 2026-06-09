use crate::syntax::path;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(path_parts: [&str; N], span: Span) -> ExprPath {
    let result = ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: path::new(path_parts, span),
    };

    return result;
}

pub(crate) fn new_generics<const N: usize>(
    path_parts: [&str; N],
    generic_argument: GenericArgument,
    span: Span,
) -> ExprPath {
    let result = ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: path::new_generics(path_parts, generic_argument, span),
    };

    return result;
}
