use proc_macro2::Ident;
use syn::{Attribute, Fields, Generics, ItemStruct, Visibility};

pub trait IStructFactory {
    fn create(&self, attrs: Vec<Attribute>, ident: Ident, fields: Fields) -> ItemStruct;
}

pub struct StructFactory;

impl IStructFactory for StructFactory {
    fn create(&self, attrs: Vec<Attribute>, ident: Ident, fields: Fields) -> ItemStruct {
        let result = ItemStruct {
            attrs,
            vis: Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics: Generics::default(),
            fields,
            semi_token: Default::default(),
        };

        return result;
    }
}
