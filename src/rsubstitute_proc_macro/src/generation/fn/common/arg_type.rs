use proc_macro2::Span;
use syn::*;

pub(crate) fn of(span: Span, r#type: Type) -> TypePath {
    let result = TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: Ident::new("Arg", span),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Token![<](span),
                    args: [GenericArgument::Type(r#type)].into_iter().collect(),
                    gt_token: Token![>](span),
                }),
            }]
            .into_iter()
            .collect(),
        },
    };

    return result;
}
