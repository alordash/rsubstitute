mod reference_type_crawling;

use crate::constants;
use syn::*;

pub(crate) fn staticify_anonymous_lifetimes(ty: &mut Type) {
    let optional_lifetimes: Vec<_> = get_all_optional_lifetimes(ty);

    for optional_lifetime in optional_lifetimes {
        if optional_lifetime.is_none() {
            *optional_lifetime = Some(constants::STATIC_LIFETIME.clone());
        }
    }
}

pub(crate) fn normalize_anonymous_lifetimes(ty: &mut Type) {
    let optional_lifetimes: Vec<_> = get_all_optional_lifetimes(ty);
    for optional_lifetime in optional_lifetimes {
        if optional_lifetime.is_none() {
            *optional_lifetime = Some(constants::DEFAULT_ARG_LIFETIME.clone());
        }
    }
}

pub(crate) fn normalize_anonymous_lifetimes_in_struct(item_struct: &mut ItemStruct) {
    for field in item_struct.fields.iter_mut() {
        normalize_anonymous_lifetimes(&mut field.ty);
    }
}

pub(crate) fn get_all_optional_lifetimes(ty: &mut Type) -> Vec<&mut Option<Lifetime>> {
    let mut result = Vec::new();
    reference_type_crawling::recursive_get_all_type_references(&mut result, ty);
    return result;
}
