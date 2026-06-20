use crate::generation::models::*;
use proc_macro2::Span;
use std::ops::Deref;
use syn::*;

pub(crate) fn generate(
    source_span: Span,
    mock_ident: Ident,
    target_struct: ItemStruct,
    setup_struct_ident: Ident,
    received_struct_ident: Ident,
    data_struct_ident: Ident,
) -> MockStructImpls {
    let result = MockStructImpls {
        target_mockable_impl: todo!(),
        deref_impl: todo!(),
        deref_mut: todo!(),
        mock_impl: todo!(),
    };

    return result;
}