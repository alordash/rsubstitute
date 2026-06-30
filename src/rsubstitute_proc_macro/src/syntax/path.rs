use crate::syntax::{generic_argument, punctuated};
use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(span: Span, path_parts: [&str; N]) -> Path {
    let result = Path {
        leading_colon: None,
        segments: path_parts
            .into_iter()
            .map(|path_part| PathSegment {
                ident: Ident::new(path_part, span),
                arguments: PathArguments::None,
            })
            .collect(),
    };
    return result;
}

pub(crate) fn new_global<const N: usize>(span: Span, path_parts: [&str; N]) -> Path {
    let mut result = new(span, path_parts);
    result.leading_colon = Some(Token![::](span));
    return result;
}

pub(crate) fn new_generics<const N: usize>(
    span: Span,
    path_parts: [&str; N],
    generic_argument: GenericArgument,
) -> Path {
    let mut result = new(span, path_parts);
    result
        .segments
        .last_mut()
        .expect("`Path` should not be empty.")
        .arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
        colon2_token: Some(Token![::](span)),
        lt_token: Token![<](span),
        args: punctuated([generic_argument]),
        gt_token: Token![>](span),
    });
    return result;
}

pub(crate) fn from_ident(ident: Ident) -> Path {
    let result = Path {
        leading_colon: None,
        segments: punctuated([PathSegment {
            ident,
            arguments: PathArguments::None,
        }]),
    };
    return result;
}

pub(crate) fn from_ident_with_generics(ident: Ident, generics: &Generics) -> Path {
    let span = ident.span();
    let result = Path {
        leading_colon: None,
        segments: punctuated([PathSegment {
            ident,
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: Some(Token![::](span)),
                lt_token: Token![<](span),
                args: generics
                    .params
                    .iter()
                    .cloned()
                    .map(generic_argument::from_param)
                    .collect(),
                gt_token: Token![>](span),
            }),
        }]),
    };
    return result;
}
