use proc_macro2::Span;
use syn::*;

const DATA_FIELD_NAME: &'static str = "__rsubstitute_data";

pub(crate) fn data_ident(span: Span) -> Ident {
    let result = Ident::new(DATA_FIELD_NAME, span);

    return result;
}

const SETUP_FIELD_NAME: &'static str = "__rsubstitute_setup";

pub(crate) fn setup_ident(span: Span) -> Ident {
    let result = Ident::new(SETUP_FIELD_NAME, span);

    return result;
}

const RECEIVED_FIELD_NAME: &'static str = "__rsubstitute_received";

pub(crate) fn received_ident(span: Span) -> Ident {
    let result = Ident::new(RECEIVED_FIELD_NAME, span);

    return result;
}
