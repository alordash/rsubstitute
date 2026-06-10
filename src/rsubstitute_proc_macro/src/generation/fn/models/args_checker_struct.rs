use syn::*;

pub(crate) struct ArgsCheckerStruct {
    pub r#type: Type,
    pub item_struct: ItemStruct,
    pub generics_info_provider_impl: ItemImpl,
    pub args_formatter_impl: ItemImpl,
}
