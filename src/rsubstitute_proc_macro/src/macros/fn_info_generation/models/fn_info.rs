use crate::macros::fn_info_generation::models::{ArgsMatcherImplInfo, ArgsMatcherInfo, CallInfo};
use crate::macros::models::FnDecl;
use proc_macro2::Ident;

pub struct FnInfo<'a> {
    pub(crate) parent: &'a FnDecl,
    pub(crate) call_info: CallInfo,
    pub(crate) args_matcher_info: ArgsMatcherInfo,
    pub(crate) args_matcher_impl_info: ArgsMatcherImplInfo,
    pub(crate) data_field_ident: Ident,
}
