use quote::format_ident;
use syn::*;

pub(crate) fn extract(arg_number: usize, fn_arg_pat_type: &PatType) -> Ident {
    let result = match &*fn_arg_pat_type.pat {
        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
        _ => format_ident!("arg_{arg_number}"),
    };
    return result;
}
