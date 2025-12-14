use proc_macro2::Ident;
use syn::{Field, FieldMutability, Type, Visibility};

pub trait IFieldFactory {
    fn create(&self, ident: Ident, ty: Type) -> Field;
}

pub struct FieldFactory;

impl IFieldFactory for FieldFactory {
    fn create(&self, ident: Ident, ty: Type) -> Field {
        let result = Field {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            mutability: FieldMutability::None,
            ident: Some(ident),
            colon_token: Some(Default::default()),
            ty,
        };
        return result;
    }
}
