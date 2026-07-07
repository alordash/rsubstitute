use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub ident: Ident,
    pub generics: Generics,
    pub maybe_argument_types: Option<Vec<Type>>,
    pub mock_struct_path: &'a Path,
    pub fn_infos: &'a [FnInfo],
}
pub(crate) fn generate(
    ctx: &Context,
    source_span: Span,
    Params {
        ident,
        generics,
        maybe_argument_types,
        mock_struct_path,
        fn_infos,
    }: Params,
) -> StaticReceivedStruct {
    let item_struct = control_struct::new_static(
        source_span,
        ident,
        generics.clone(),
        maybe_argument_types,
        ControlType::Received,
    );
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);
    let item_impl = received_impl::generate(
        ctx,
        source_span,
        received_impl::Params {
            received_struct_path: path.clone(),
            generics,
            mock_struct_path,
            fn_infos,
            is_static: true,
        },
    );

    let result = StaticReceivedStruct {
        path,
        item_struct,
        item_impl,
    };
    return result;
}
