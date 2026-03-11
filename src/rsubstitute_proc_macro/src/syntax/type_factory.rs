// TODO - replace everywhere with `use syn::*`
use crate::constants;
use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use std::sync::Arc;
use syn::*;

pub(crate) trait ITypeFactory {
    fn create(&self, ident: Ident) -> Type;

    fn create_with_generics(&self, ident: Ident, generics: Generics) -> Type;

    fn create_from_struct(&self, item_struct: &ItemStruct) -> Type;

    fn wrap_in(&self, ty: Type, wrapper: Ident) -> Type;

    fn wrap_in_arc(&self, ty: Type) -> Type;

    fn reference(&self, ty: Type, lifetime: Option<Lifetime>) -> Type;

    fn mut_reference(&self, ty: Type, lifetime: Option<Lifetime>) -> Type;

    fn phantom_data_lifetime(&self, lifetime: Lifetime) -> Type;

    fn phantom_data(&self, ty_ident: Ident) -> Type;
}

pub(crate) struct TypeFactory {
    pub path_factory: Arc<dyn IPathFactory>,
}

impl ITypeFactory for TypeFactory {
    fn create(&self, ident: Ident) -> Type {
        let path = self.path_factory.create(ident);
        let result = Type::Path(TypePath { qself: None, path });
        return result;
    }

    fn create_with_generics(&self, ident: Ident, generics: Generics) -> Type {
        let path = self.path_factory.create_with_generics(ident, generics);
        let result = Type::Path(TypePath { qself: None, path });
        return result;
    }

    fn create_from_struct(&self, item_struct: &ItemStruct) -> Type {
        let result =
            self.create_with_generics(item_struct.ident.clone(), item_struct.generics.clone());
        return result;
    }

    fn wrap_in(&self, ty: Type, wrapper: Ident) -> Type {
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: wrapper,
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [GenericArgument::Type(ty)].into_iter().collect(),
                        gt_token: Default::default(),
                    }),
                }]
                .into_iter()
                .collect(),
            },
        });
        return result;
    }

    fn wrap_in_arc(&self, ty: Type) -> Type {
        let result = self.wrap_in(ty, constants::ARC_IDENT.clone());
        return result;
    }

    fn reference(&self, ty: Type, lifetime: Option<Lifetime>) -> Type {
        let result = Type::Reference(TypeReference {
            and_token: Default::default(),
            lifetime,
            mutability: None,
            elem: Box::new(ty.clone()),
        });
        return result;
    }

    fn mut_reference(&self, ty: Type, lifetime: Option<Lifetime>) -> Type {
        let result = Type::Reference(TypeReference {
            and_token: Default::default(),
            lifetime,
            mutability: Some(Default::default()),
            elem: Box::new(ty.clone()),
        });
        return result;
    }

    fn phantom_data_lifetime(&self, lifetime: Lifetime) -> Type {
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: constants::PHANTOM_DATA_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [GenericArgument::Type(
                            self.reference(constants::VOID_TYPE.clone(), Some(lifetime)),
                        )]
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

    fn phantom_data(&self, ty_ident: Ident) -> Type {
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: constants::PHANTOM_DATA_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [GenericArgument::Type(self.create(ty_ident))]
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
