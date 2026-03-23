mod lifetime_visitor;

use crate::constants;
use crate::syntax::lifetime::lifetime_visitor::*;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn staticify_anonymous_lifetimes(ty: &mut Type) {
    let static_lifetime = constants::STATIC_LIFETIME.clone();
    let mut lifetime_replacer =
        LifetimeReplacer::new(ReplacementStrategy::ReplaceOnlyOptional(&static_lifetime));
    lifetime_replacer.visit_type_mut(ty);
}

pub(crate) fn placehold_anonymouys_lifetimes(ty: &mut Type) {
    let placeholder_lifetime = constants::PLACEHOLDER_LIFETIME.clone();
    let mut lifetime_replacer = LifetimeReplacer::new(ReplacementStrategy::ReplaceOnlyOptional(
        &placeholder_lifetime,
    ));
    lifetime_replacer.visit_type_mut(ty);
}

pub(crate) fn set_all_lifetimes(ty: &mut Type, new_lifetime: &Lifetime) {
    let mut lifetime_replacer =
        LifetimeReplacer::new(ReplacementStrategy::ReplaceAll(new_lifetime));
    lifetime_replacer.visit_type_mut(ty);
}

pub(crate) fn set_all_lifetimes_in_generics(generics: &mut Generics, new_lifetime: &Lifetime) {
    let mut lifetime_replacer =
        LifetimeReplacer::new(ReplacementStrategy::ReplaceAll(new_lifetime));
    lifetime_replacer.visit_generics_mut(generics);
}

pub(crate) fn remove_all_lifetimes(ty: &mut Type) {
    let mut lifetime_replacer = LifetimeReplacer::new(ReplacementStrategy::RemoveAll);
    lifetime_replacer.visit_type_mut(ty);
}
