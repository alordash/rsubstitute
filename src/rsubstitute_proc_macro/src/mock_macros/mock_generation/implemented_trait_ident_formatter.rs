use proc_macro2::Ident;
use quote::format_ident;

pub trait IImplementedTraitIdentFormatter {
    fn format_for_field(&self, implemented_trait_ident: &Ident) -> Ident;
}

pub(crate) struct ImplementedTraitIdentFormatter;

impl IImplementedTraitIdentFormatter for ImplementedTraitIdentFormatter {
    fn format_for_field(&self, implemented_trait_ident: &Ident) -> Ident {
        format_ident!("as_{}", implemented_trait_ident)
    }
}