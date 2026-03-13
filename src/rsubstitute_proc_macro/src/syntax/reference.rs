mod reference_lifetime;
mod reference_type_crawling;
pub(crate) use reference_lifetime::*;

use crate::constants;
use syn::*;

pub(crate) fn staticify_anonymous_lifetimes(ty: &mut Type) {
    let optional_lifetimes = get_all_optional_lifetimes(ty);

    for mut optional_lifetime in optional_lifetimes {
        if optional_lifetime.is_none() {
            optional_lifetime.set(constants::STATIC_LIFETIME.clone());
        }
    }
}

pub(crate) fn normalize_anonymous_lifetimes(ty: &mut Type) {
    let optional_lifetimes = get_all_optional_lifetimes(ty);
    for mut optional_lifetime in optional_lifetimes {
        if optional_lifetime.is_none() {
            optional_lifetime.set(constants::DEFAULT_ARG_LIFETIME.clone());
        }
    }
}

pub(crate) fn normalize_anonymous_lifetimes_in_struct(item_struct: &mut ItemStruct) {
    for field in item_struct.fields.iter_mut() {
        normalize_anonymous_lifetimes(&mut field.ty);
    }
}

pub(crate) fn set_all_lifetimes(ty: &mut Type, new_lifetime: &Lifetime) {
    let optional_lifetimes = get_all_optional_lifetimes(ty);
    for mut optional_lifetime in optional_lifetimes {
        optional_lifetime.set(new_lifetime.clone());
    }
}

pub(crate) fn get_all_optional_lifetimes(ty: &mut Type) -> Vec<ReferenceLifetime> {
    let mut result = Vec::new();
    reference_type_crawling::recursive_get_all_type_references(&mut result, ty);
    return result;
}
