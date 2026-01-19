use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::models::FnDecl;
use proc_macro2::Ident;

pub struct FnInfo<'a> {
    pub(crate) parent: &'a FnDecl,
    pub(crate) call_struct: CallStruct,
    pub(crate) call_arg_infos_provider_impl: CallArgInfosProviderImpl,
    pub(crate) args_checker_struct: ArgsCheckerStruct,
    pub(crate) args_checker_impl: ArgsCheckerTraitImpl,
    pub(crate) data_field_ident: Ident,
}
