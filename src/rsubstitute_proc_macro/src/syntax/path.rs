use proc_macro2::Span;
use syn::*;

pub(crate) fn new<const N: usize>(path_parts: [&str; N], span: Span) -> Path {
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

pub(crate) fn new_generics<const N: usize>(
    path_parts: [&str; N],
    generic_argument: GenericArgument,
    span: Span,
) -> Path {
    let mut result = new(path_parts, span);
    result
        .segments
        .last_mut()
        .expect("`Path` should not be empty.")
        .arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
        colon2_token: Some(Token![::](span)),
        lt_token: Token![<](span),
        args: [generic_argument].into_iter().collect(),
        gt_token: Token![>](span),
    });
    return result;
}
