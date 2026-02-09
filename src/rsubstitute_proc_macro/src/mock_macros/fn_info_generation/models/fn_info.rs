use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::models::FnDecl;
use proc_macro2::Ident;

// TODO - make all models internal
pub(crate) struct FnInfo {
    // TODO - remove this field or use it instead of FnDecl everywhere
    pub parent: FnDecl,
    pub call_struct: CallStruct,
    pub call_arg_infos_provider_impl: CallArgInfosProviderImpl,
    pub args_checker_struct: ArgsCheckerStruct,
    pub args_checker_impl: ArgsCheckerTraitImpl,
    pub data_field_ident: Ident,
    pub maybe_base_caller_impl: Option<BaseCallerImpl>,
}
