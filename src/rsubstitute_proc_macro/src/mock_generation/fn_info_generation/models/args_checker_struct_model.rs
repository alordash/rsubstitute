use syn::*;
use crate::mock_generation::fn_info_generation::models::*;

pub(crate) struct ArgsCheckerStruct {
    pub generics_info_provider_impl: ItemImpl,
    pub item_struct: ItemStruct,
    pub ty_path: TypePath,
    pub args_checker_trait_impl: ArgsCheckerTraitImpl,
    pub args_checker_args_formatter_trait_impl: ItemImpl,
}
