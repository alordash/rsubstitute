use crate::generation::mock_controls::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub struct_ident: &'a Ident,
    pub struct_generics: Generics,
    pub trait_ident: &'a Ident,
    pub trait_generics: Generics,
    pub maybe_common_where_clause: Option<WhereClause>,
    pub control_type: ControlType,
    pub is_static: bool,
}
pub(crate) fn generate(
    span: Span,
    Params {
        struct_ident,
        struct_generics,
        trait_ident,
        trait_generics,
        maybe_common_where_clause,
        control_type,
        is_static,
    }: Params,
) -> ItemImpl {
    let result = ItemImpl {};
    return result;
}
