use crate::constants;
use syn::*;

pub(crate) trait IArgTypeFactory {
    fn create(&self, ty: Type) -> Type;
}

pub(crate) struct ArgTypeFactory;

impl IArgTypeFactory for ArgTypeFactory {
    fn create(&self, ty: Type) -> Type {
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: Default::default(),
                segments: [PathSegment {
                    ident: constants::ARG_TYPE_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [
                            GenericArgument::Lifetime(
                                constants::DEFAULT_ARG_LIFETIME.clone(),
                            ),
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
        });
        return result;
    }
}
