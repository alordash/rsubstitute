use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::common::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::syntax::*;
use proc_macro2::Span;
use std::borrow::Borrow;
use syn::*;

pub(crate) struct Params<'a, T: Borrow<FnInfo>> {
    pub ident: Ident,
    pub generics: Generics,
    pub mock_struct_path: &'a Path,
    pub fn_infos: &'a [T],
    pub maybe_trait_ident: Option<Ident>,
}
pub(crate) fn generate<T: Borrow<FnInfo>>(
    ctx: &Context,
    span: Span,
    Params {
        ident,
        generics,
        mock_struct_path,
        fn_infos,
        maybe_trait_ident,
    }: Params<T>,
) -> SetupStruct {
    let item_struct =
        control_struct::new(span, ident, generics, ControlType::Setup, maybe_trait_ident);
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);
    let item_impl = setup_impl::generate(
        ctx,
        span,
        setup_impl::Params {
            setup_struct_path: path.clone(),
            generics: item_struct.generics.clone(),
            mock_struct_path,
            fn_infos,
            for_static_fn: false,
            is_static: false,
        },
    );

    let result = SetupStruct {
        path,
        item_struct,
        item_impl,
    };
    return result;
}
