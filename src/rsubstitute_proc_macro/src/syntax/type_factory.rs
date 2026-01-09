use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use std::sync::Arc;
// TODO - replace everywhere with `use syn::*`
use crate::constants;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Generics, ItemStruct, Path, PathArguments,
    PathSegment, Type, TypePath,
};

pub trait ITypeFactory {
    fn create(&self, ident: Ident) -> Type;

    fn create_with_generics(&self, ident: Ident, generics: Generics) -> Type;

    // TODO - replace most `create_with_generics` with this method
    fn create_from_struct(&self, item_struct: &ItemStruct) -> Type;

    fn wrap_in(&self, ty: Type, wrapper: Ident) -> Type;

    fn wrap_in_arc(&self, ty: Type) -> Type;
}

pub struct TypeFactory {
    pub(crate) path_factory: Arc<dyn IPathFactory>,
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
        self.create_with_generics(item_struct.ident.clone(), item_struct.generics.clone())
    }

    fn wrap_in_arc(&self, ty: Type) -> Type {
        let result = self.wrap_in(ty, constants::ARC_IDENT.clone());
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
}
