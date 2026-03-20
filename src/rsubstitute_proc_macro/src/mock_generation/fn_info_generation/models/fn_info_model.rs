use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::models::FnDecl;
use proc_macro2::Ident;
use syn::ItemImpl;

pub(crate) struct FnInfo {
    pub parent: FnDecl,
    pub call_struct: CallStruct,
    pub args_checker_struct: ArgsCheckerStruct,
    pub args_checker_trait_impl: ArgsCheckerTraitImpl,
    pub args_checker_args_formatter_trait_impl: ItemImpl,
    pub data_field_ident: Ident,
}
