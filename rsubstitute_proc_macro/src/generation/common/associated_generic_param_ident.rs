use proc_macro2::Ident;
use quote::format_ident;

fn format(trait_ident: &Ident, generic_param_ident: &Ident) -> Ident {
    format_ident!("{trait_ident}_{generic_param_ident}")
}
