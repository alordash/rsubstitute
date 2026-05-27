use crate::syntax::constants::{COLON, PLUS};
use quote::format_ident;
use syn::*;
use crate::syntax::type_param_bound;

pub(crate) fn to_ident(type_param: &TypeParam) -> Ident {
    let type_param_ident = &type_param.ident;
    let bounds_idents: Vec<_> = type_param
        .bounds
        .iter()
        .map(type_param_bound::to_ident)
        .map(|x| x.to_string())
        .collect();
    let bounds_idents_string = bounds_idents.join(PLUS);
    let result = format_ident!("{type_param_ident}{COLON}{bounds_idents_string}");
    return result;
}
