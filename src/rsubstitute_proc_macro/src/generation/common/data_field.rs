use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params {
    pub mock_struct_path: Path,
    pub public: bool,
}
pub(crate) fn new(
    span: Span,
    Params {
        mock_struct_path,
        public,
    }: Params,
) -> Field {
    let result = Field {
        attrs: Vec::new(),
        vis: if public {
            Visibility::Public(Token![pub](span))
        } else {
            Visibility::Inherited
        },
        mutability: FieldMutability::None,
        ident: Some(Ident::new("data", span)),
        colon_token: Some(Token![:](span)),
        ty: Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: Some(Token![::](span)),
                segments: rsubstitute_punctuated(
                    span,
                    [PathSegment {
                        ident: Ident::new("SharedMockData", span),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Token![<](span),
                            args: punctuated([GenericArgument::Type(Type::Path(TypePath {
                                qself: None,
                                path: mock_struct_path,
                            }))]),
                            gt_token: Token![>](span),
                        }),
                    }],
                ),
            },
        }),
    };
    return result;
}
