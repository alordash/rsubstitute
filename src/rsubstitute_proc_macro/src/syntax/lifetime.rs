use crate::syntax::constants::APOSTROPHE;
use quote::format_ident;
use syn::*;

pub(crate) fn to_ident(lifetime: &Lifetime) -> Ident {
    let result = format_ident!("{APOSTROPHE}{}", lifetime.ident);
    return result;
}
