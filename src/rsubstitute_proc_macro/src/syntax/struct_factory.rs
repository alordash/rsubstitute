use crate::constants;
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IStructFactory {
    fn create_with_default_lifetime(
        &self,
        attrs: Vec<Attribute>,
        ident: Ident,
        fields: FieldsNamed,
    ) -> ItemStruct;
}

pub struct StructFactory;

impl IStructFactory for StructFactory {
    fn create_with_default_lifetime(
        &self,
        attrs: Vec<Attribute>,
        ident: Ident,
        mut fields_named: FieldsNamed,
    ) -> ItemStruct {
        fields_named
            .named
            .insert(0, constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD.clone());
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
