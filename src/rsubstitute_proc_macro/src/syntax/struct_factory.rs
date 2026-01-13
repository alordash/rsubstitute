use crate::constants;
use proc_macro2::Ident;
use syn::*;

pub trait IStructFactory {
    fn create(&self, attrs: Vec<Attribute>, ident: Ident, fields_named: FieldsNamed) -> ItemStruct;

    fn create_for_static(
        &self,
        attrs: Vec<Attribute>,
        ident: Ident,
        fields_named: FieldsNamed,
    ) -> ItemStruct;
}

pub struct StructFactory;

impl IStructFactory for StructFactory {
    fn create(&self, attrs: Vec<Attribute>, ident: Ident, fields_named: FieldsNamed) -> ItemStruct {
        let with_default_arg_field_lifetime_generic = true;
        let result = self.create(
            attrs,
            ident,
            fields_named,
            with_default_arg_field_lifetime_generic,
        );

        return result;
    }

    fn create_for_static(
        &self,
        attrs: Vec<Attribute>,
        ident: Ident,
        fields_named: FieldsNamed,
    ) -> ItemStruct {
        let with_default_arg_field_lifetime_generic = false;
        let result = self.create(
            attrs,
            ident,
            fields_named,
            with_default_arg_field_lifetime_generic,
        );

        return result;
    }
}

impl StructFactory {
    fn create(
        &self,
        attrs: Vec<Attribute>,
        ident: Ident,
        fields_named: FieldsNamed,
        with_default_arg_field_lifetime_generic: bool,
    ) -> ItemStruct {
        let generics = if with_default_arg_field_lifetime_generic {
            constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC.clone()
        } else {
            Generics::default()
        };
        let result = ItemStruct {
            attrs,
            vis: Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics,
            fields: Fields::Named(fields_named),
            semi_token: Default::default(),
        };

        return result;
    }
}
