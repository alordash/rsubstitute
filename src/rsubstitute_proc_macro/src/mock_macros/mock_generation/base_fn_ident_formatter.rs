use crate::constants;
use proc_macro2::Ident;
use quote::format_ident;

pub(crate) trait IBaseFnIdentFormatter {
    fn format(&self, fn_ident: &Ident) -> Ident;
}

pub(crate) struct BaseFnIdentFormatter;

impl IBaseFnIdentFormatter for BaseFnIdentFormatter {
    fn format(&self, fn_ident: &Ident) -> Ident {
        format_ident!("{}_{}", constants::BASE_FN_IDENT_PREFIX, fn_ident)
    }
}
