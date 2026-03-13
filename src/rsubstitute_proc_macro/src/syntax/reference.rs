mod reference_type_crawling;

use crate::constants;
use syn::*;

pub(crate) fn staticify_anonymous_lifetimes(ty: &mut Type) {
    let optional_lifetimes = get_all_optional_lifetimes(ty);

    for optional_lifetime in optional_lifetimes {
        if optional_lifetime.is_none() {
            *optional_lifetime = Some(constants::STATIC_LIFETIME.clone());
        }
    }
}

pub(crate) fn normalize_anonymous_lifetimes(ty: &mut Type) {
    let optional_lifetimes = get_all_optional_lifetimes(ty);
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

pub(crate) fn anonymize_normal_lifetimes(ty: &mut Type) {
    let optional_lifetimes = get_all_optional_lifetimes(ty);
    for optional_lifetime in optional_lifetimes {
        if let Some(lifetime) = optional_lifetime
            && lifetime.ident.to_string() == constants::DEFAULT_ARG_LIFETIME_NAME
        {
            // lifetime.ident = constants::ANONYMOUS_LIFETIME_IDENT.clone();
        }
    }
}

pub(crate) fn get_all_optional_lifetimes<'a>(ty: &'a mut Type) -> Vec<&'a mut Option<Lifetime>> {
    let mut result = Vec::new();
    let mut visitor =
        |type_reference: &'a mut TypeReference| result.push(&mut type_reference.lifetime);
    reference_type_crawling::recursive_visit_all_type_references(&mut visitor, ty);
    return result;
}

pub(crate) fn visit_all_optional_lifetimes<'a>(
    ty: &'a mut Type,
    visitor: &mut dyn FnMut(&'a mut TypeReference),
) {
    reference_type_crawling::recursive_visit_all_type_references(visitor, ty);
}
