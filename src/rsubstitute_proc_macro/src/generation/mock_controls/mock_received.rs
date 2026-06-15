use super::*;
use crate::generation::mock_controls::constants::data_ident;
use crate::generation::mock_controls::models::*;
use crate::generation::r#fn::models::*;
use crate::generation::*;
use crate::preparation::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params<'a> {
    pub ctx: &'a Context,
    pub source_span: Span,
    pub target_ident: Ident,
    pub mock_type: Type,
    pub mock_data_ident: Ident,
    pub stores_mock_data: bool,
    pub fn_infos: &'a [FnInfo],
}

pub(crate) fn generate(
    crate::generation::mock_controls::mock_setup::Params {
        ctx,
        source_span,
        target_ident,
        mock_type,
        mock_data_ident,
        stores_mock_data,
        fn_infos,
    }: crate::generation::mock_controls::mock_setup::Params,
) -> MockReceived {

}