use crate::syntax::{generic_argument, punctuated};
use proc_macro2::Span;
use syn::*;

#[inline]
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

#[inline]
pub(crate) fn new_global<const N: usize>(span: Span, path_parts: [&str; N]) -> Path {
    let mut result = new(span, path_parts);
    result.leading_colon = Some(Token![::](span));
    return result;
}

#[inline]
pub(crate) fn new_generics<const N_PATH: usize, const N_GENERICS: usize>(
    span: Span,
    path_parts: [&str; N_PATH],
    generic_arguments: [GenericArgument; N_GENERICS],
) -> Path {
    let mut result = new(span, path_parts);
    result
        .segments
        .last_mut()
        .expect("`Path` should not be empty.")
        .arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
        colon2_token: Some(Token![::](span)),
        lt_token: Token![<](span),
        args: generic_arguments.into_iter().collect(),
        gt_token: Token![>](span),
    });
    return result;
}

#[inline]
pub(crate) fn new_generics_global<const N: usize, const N_GENERICS: usize>(
    span: Span,
    path_parts: [&str; N],
    generic_arguments: [GenericArgument; N_GENERICS],
) -> Path {
    let mut result = new_generics(span, path_parts, generic_arguments);
    result.leading_colon = Some(Token![::](span));
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

pub(crate) fn from_base_path_with_ident(base: &Path, ident: Ident) -> Path {
    let mut result = base.clone();
    if let Some(last_segment) = result.segments.last_mut() {
        last_segment.ident = ident;
    }
    result.segments = result.segments.into_iter().last().into_iter().collect();
    return result;
}

pub(crate) fn last_ident(path: &Path) -> Ident {
    let result = path
        .segments
        .last()
        .expect("`path::last` expects given path to have at least one segment.")
        .ident
        .clone();
    return result;
}

pub(crate) fn equal(left: &Path, right: &Path) -> bool {
    if left.segments.len() != right.segments.len() {
        return false;
    }

    let result = left
        .segments
        .iter()
        .zip(right.segments.iter())
        .all(|(left_segment, right_segment)| left_segment.ident == right_segment.ident);
    return result;
}

pub(crate) fn remove_lifetime_generic_arguments(mut path: Path) -> Path {
    if let Some(last_segment) = path.segments.last_mut()
        && let PathArguments::AngleBracketed(angle_bracketed_path_arguments) =
            &mut last_segment.arguments
    {
        angle_bracketed_path_arguments.args =
            core::mem::take(&mut angle_bracketed_path_arguments.args)
                .into_iter()
                .filter(|x| match x {
                    GenericArgument::Lifetime(_) => false,
                    _ => true,
                })
                .collect();
    }
    return path;
}
