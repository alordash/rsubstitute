use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn of(span: Span, r#type: Type) -> TypePath {
    let result = TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: punctuated([PathSegment {
                ident: Ident::new("Arg", span),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Token![<](span),
                    args: punctuated([GenericArgument::Type(r#type)]),
                    gt_token: Token![>](span),
                }),
            }]),
        },
    };

    return result;
}
