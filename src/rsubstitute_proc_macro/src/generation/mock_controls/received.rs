use crate::common::models::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::syntax::path;
use proc_macro2::Span;
use std::borrow::Borrow;
use syn::*;

pub(crate) struct Params<'a, T: Borrow<FnInfo>> {
    pub ident: Ident,
    pub generics: Generics,
    pub mock_struct_path: &'a Path,
    pub fn_infos: &'a [T],
}
pub(crate) fn generate<T: Borrow<FnInfo>>(
    ctx: &Context,
    span: Span,
    Params {
        ident,
        generics,
        mock_struct_path,
        fn_infos,
    }: Params<T>,
) -> ReceivedStruct {
    let item_struct = control_struct::new(span, ident, generics.clone(), ControlType::Received);
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);
    let clone_impl = clone_impl::generate(span, generics, path.clone(), &item_struct.fields);
    let item_impl = received_impl::generate(
        ctx,
        span,
        received_impl::Params {
            received_struct_path: path.clone(),
            generics: item_struct.generics.clone(),
            mock_struct_path,
            fn_infos,
            for_static_fn: false,
            is_static: false,
        },
    );

    let result = ReceivedStruct {
        path,
        item_struct,
        clone_impl,
        item_impl,
    };
    return result;
}
