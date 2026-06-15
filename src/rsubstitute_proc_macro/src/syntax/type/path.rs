use crate::syntax::path;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(span: Span, path_parts: [&str; N]) -> TypePath {
    let result = TypePath {
        qself: None,
        path: path::new(span, path_parts),
    };

    return result;
}
