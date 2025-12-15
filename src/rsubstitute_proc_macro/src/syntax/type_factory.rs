use crate::syntax::IPathFactory;
use proc_macro2::Ident;
use std::rc::Rc;
use syn::{Type, TypePath};

pub trait ITypeFactory {
    fn create(&self, ident: Ident) -> Type;
}

pub struct TypeFactory {
    path_factory: Rc<dyn IPathFactory>,
}

impl ITypeFactory for TypeFactory {
    fn create(&self, ident: Ident) -> Type {
        let path = self.path_factory.create(ident);
        let result = Type::Path(TypePath { qself: None, path });
        return result;
    }
}
