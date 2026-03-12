use proc_macro2::Ident;
use syn::*;

pub(crate) fn create(
    attrs: Vec<Attribute>,
    ident: Ident,
    generics: Generics,
    fields_named: FieldsNamed,
) -> ItemStruct {
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
