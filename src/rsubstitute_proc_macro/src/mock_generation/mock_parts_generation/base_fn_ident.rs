use crate::constants;
use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn format(fn_ident: &Ident) -> Ident {
    format_ident!("{}_{}", constants::BASE_FN_IDENT_PREFIX, fn_ident)
}
