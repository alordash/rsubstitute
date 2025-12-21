use crate::mock_macros::constants;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, Type,
    TypePath,
};

pub trait IArgTypeFactory {
    fn create(&self, type_path: &TypePath) -> Type;
}

pub struct ArgTypeFactory;

impl IArgTypeFactory for ArgTypeFactory {
    fn create(&self, type_path: &TypePath) -> Type {
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: Default::default(),
                segments: [PathSegment {
                    ident: constants::ARG_TYPE_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: Some(Default::default()),
                        lt_token: Default::default(),
                        args: [GenericArgument::Type(Type::Path(type_path.clone()))]
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
