use crate::constants;
use proc_macro2::Ident;
use syn::*;

pub trait IStructFactory {
    fn create(&self, attrs: Vec<Attribute>, ident: Ident, fields_named: FieldsNamed) -> ItemStruct;
}

pub struct StructFactory;

impl IStructFactory for StructFactory {
    fn create(&self, attrs: Vec<Attribute>, ident: Ident, fields_named: FieldsNamed) -> ItemStruct {
        let result = ItemStruct {
            attrs,
            vis: Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics: constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC.clone(),
            fields: Fields::Named(fields_named),
            semi_token: Default::default(),
        };

        return result;
    }
}
