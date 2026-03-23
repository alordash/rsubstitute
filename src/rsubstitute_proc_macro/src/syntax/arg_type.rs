use crate::constants;
use syn::*;

pub(crate) fn create(ty: Type) -> TypePath {
    let result = TypePath {
        qself: None,
        path: Path {
            leading_colon: Default::default(),
            segments: [PathSegment {
                ident: constants::ARG_TYPE_IDENT.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [
                        GenericArgument::Type(ty),
                    ]
                    .into_iter()
                    .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    };
    return result;
}
