use proc_macro2::Span;
use syn::*;

const DATA_FIELD: &'static str = "data";

pub(crate) fn data_ident(span: Span) -> Ident {
    let result = Ident::new(DATA_FIELD, span);

    return result;
}
