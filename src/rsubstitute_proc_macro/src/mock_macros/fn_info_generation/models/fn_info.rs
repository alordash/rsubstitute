use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::models::FnDecl;
use proc_macro2::Ident;

// TODO - make all models internal
pub struct FnInfo<'a> {
    // TODO - remove this field or use it instead of FnDecl everywhere
    pub(crate) parent: &'a FnDecl,
    pub(crate) call_struct: CallStruct,
    pub(crate) call_arg_infos_provider_impl: CallArgInfosProviderImpl,
    pub(crate) args_checker_struct: ArgsCheckerStruct,
    pub(crate) args_checker_impl: ArgsCheckerTraitImpl,
    pub(crate) data_field_ident: Ident,
    pub(crate) maybe_base_caller_impl: Option<BaseCallerImpl>
}
