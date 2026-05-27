use crate::syntax::generic_param;
use crate::syntax::constants::{COMMA, GREATER, LESS};
use quote::format_ident;
use syn::*;

pub(crate) fn to_ident(bound_lifetimes: &BoundLifetimes) -> Ident {
    let generic_params: Vec<_> = bound_lifetimes
        .lifetimes
        .iter()
        .map(generic_param::to_ident)
        .map(|x| x.to_string())
        .collect();
    let generic_params_string = generic_params.join(COMMA);
    let result = format_ident!("for{LESS}{generic_params_string}{GREATER}");
    return result;
}
