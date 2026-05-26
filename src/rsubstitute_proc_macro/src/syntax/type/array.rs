use quote::format_ident;
use syn::*;

pub(crate) fn to_ident(type_array: &TypeArray) -> Ident {
    let elem_type_ident = super::to_ident(&type_array.elem);
    return format_ident!("__array_{elem_type_ident}")
}