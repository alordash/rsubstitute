use crate::mock_macros::fn_info_generation::models::{ArgsCheckerImplInfo, ArgsCheckerInfo, CallInfo};
use crate::mock_macros::models::FnDecl;
use proc_macro2::Ident;

pub struct FnInfo<'a> {
    pub(crate) parent: &'a FnDecl,
    pub(crate) call_info: CallInfo,
    pub(crate) args_checker_info: ArgsCheckerInfo,
    pub(crate) args_checker_impl_info: ArgsCheckerImplInfo,
    pub(crate) data_field_ident: Ident,
}
