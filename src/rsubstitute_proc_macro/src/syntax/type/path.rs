use crate::syntax::{path, punctuated};
use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(span: Span, path_parts: [&str; N]) -> TypePath {
    let result = TypePath {
        qself: None,
        path: path::new(span, path_parts),
    };

    return result;
}

pub(crate) fn from_ident(ident: Ident) -> TypePath {
    let result = TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: punctuated([PathSegment {
                ident,
                arguments: PathArguments::None,
            }]),
        },
    };

    return result;
}
