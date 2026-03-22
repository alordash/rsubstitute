use syn::*;

pub(crate) struct ArgsCheckerStruct {
    pub generics_info_provider_impl: ItemImpl,
    pub item_struct: ItemStruct,
    pub ty_path: TypePath,
    pub args_checker_trait_impl: ItemImpl,
    pub args_checker_args_formatter_trait_impl: ItemImpl,
}
