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
        mut fields: FieldsNamed,
    ) -> ItemStruct {
        fields.named.insert(0, )
        let result = ItemStruct {
            attrs,
            vis: Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics: Generics {
                lt_token: Some(Default::default()),
                params: [GenericParam::Lifetime(LifetimeParam {
                    attrs: Vec::new(),
                    lifetime: constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                })]
                .into_iter()
                .collect(),
                gt_token: Some(Default::default()),
                where_clause: None,
            },
            fields,
            semi_token: Default::default(),
        };

        return result;
    }
}
