use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use syn::*;

pub(crate) fn try_map_generic_param(generic_param: &GenericParam) -> Option<Field> {
    match generic_param {
        GenericParam::Lifetime(generic_lifetime) => Some(field::create(
            format_phantom_field_name(&generic_lifetime.lifetime.ident),
            r#type::phantom_data_lifetime(generic_lifetime.lifetime.clone()),
        )),
        GenericParam::Type(generic_type) => Some(field::create(
            format_phantom_field_name(&generic_type.ident),
            r#type::phantom_data(generic_type.ident.clone()),
        )),
        _ => None,
    }
}

fn format_phantom_field_name(ident: &Ident) -> Ident {
    format_ident!("_phantom_{ident}")
}
