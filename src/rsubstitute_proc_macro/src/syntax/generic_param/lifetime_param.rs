use crate::syntax::lifetime;
use crate::syntax::constants::{COLON, PLUS};
use quote::format_ident;
use syn::*;

pub(crate) fn to_ident(lifetime_param: &LifetimeParam) -> Ident {
    let lifetime_ident = lifetime::to_ident(&lifetime_param.lifetime);
    let bounds_idents: Vec<_> = lifetime_param
        .bounds
        .iter()
        .map(lifetime::to_ident)
        .map(|x| x.to_string())
        .collect();
    let bounds_idents_string = bounds_idents.join(PLUS);
    let result = format_ident!("{lifetime_ident}{COLON}_{bounds_idents_string}");
    return result;
}
