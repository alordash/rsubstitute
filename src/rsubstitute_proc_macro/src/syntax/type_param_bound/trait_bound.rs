use crate::syntax::constants::IDENT_SEGMENTS_SEPARATOR;
use crate::syntax::{bound_lifetimes, path};
use quote::format_ident;
use syn::*;

pub mod trait_bound_modifier;

pub(crate) fn to_ident(trait_bound: &TraitBound) -> Ident {
    let maybe_modifier_ident = trait_bound_modifier::to_maybe_ident(&trait_bound.modifier);
    let maybe_lifetimes_ident = trait_bound
        .lifetimes
        .as_ref()
        .map(bound_lifetimes::to_ident)
        .map(|x| format_ident!("{x}{IDENT_SEGMENTS_SEPARATOR}"));
    let path_ident = path::to_ident(&trait_bound.path, IDENT_SEGMENTS_SEPARATOR);
    let mut result = path_ident;
    if let Some(modifier_ident) = maybe_modifier_ident {
        result = format_ident!("{modifier_ident}{result}");
    }
    if let Some(lifetimes_ident) = maybe_lifetimes_ident {
        result = format_ident!("{lifetimes_ident}{result}");
    }
    return result;
}
