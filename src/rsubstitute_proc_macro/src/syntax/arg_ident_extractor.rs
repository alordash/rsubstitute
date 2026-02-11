use quote::format_ident;
use syn::*;

pub trait IArgIdentExtractor {
    fn extract(&self, arg_number: usize, fn_arg_pat_type: &PatType) -> Ident;
}

pub(crate) struct ArgIdentExtractor;

impl IArgIdentExtractor for ArgIdentExtractor {
    fn extract(&self, arg_number: usize, fn_arg_pat_type: &PatType) -> Ident {
        let result = match &*fn_arg_pat_type.pat {
            Pat::Ident(pat_ident) => pat_ident.ident.clone(),
            _ => format_ident!("arg_{arg_number}"),
        };
        return result;
    }
}
