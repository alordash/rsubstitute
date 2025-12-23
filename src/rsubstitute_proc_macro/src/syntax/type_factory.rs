use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use std::rc::Rc;
// TODO - replace everywhere with `use syn::*`
use syn::{Generics, ItemStruct, Type, TypePath};

pub trait ITypeFactory {
    fn create(&self, ident: Ident) -> Type;

    fn create_with_generics(&self, ident: Ident, generics: Generics) -> Type;

    // TODO - replace most `create_with_generics` with this method
    fn create_from_struct(&self, item_struct: &ItemStruct) -> Type;
}

pub struct TypeFactory {
    pub(crate) path_factory: Rc<dyn IPathFactory>,
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
}
