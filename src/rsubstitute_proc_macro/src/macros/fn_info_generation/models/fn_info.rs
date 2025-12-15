use crate::macros::fn_info_generation::models::{ArgsMatcherImplInfo, ArgsMatcherInfo, CallInfo};
use crate::macros::models::FnDecl;

pub struct FnInfo<'a> {
    pub(crate) parent: &'a FnDecl,
    pub(crate) call_info: CallInfo<'a>,
    pub(crate) args_matcher_info: ArgsMatcherInfo<'a>,
    pub(crate) args_matcher_impl_info: ArgsMatcherImplInfo<'a>,
}
