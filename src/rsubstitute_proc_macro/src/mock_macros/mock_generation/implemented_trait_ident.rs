use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn format_for_field(implemented_trait_ident: &Ident) -> Ident {
    format_ident!("as_{}", implemented_trait_ident)
}
