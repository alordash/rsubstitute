use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::models::FnDecl;
use proc_macro2::Ident;

pub(crate) struct FnInfo {
    pub parent: FnDecl,
    pub call_struct: CallStruct,
    pub args_checker_struct: ArgsCheckerStruct,
    pub data_field_ident: Ident,
}
