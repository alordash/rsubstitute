use syn::*;

pub(crate) struct CallStruct {
    pub r#type: Type,
    pub item_struct: ItemStruct,
    pub generics_info_provider_impl: ItemImpl,
    pub args_infos_provider_impl: ItemImpl,
    pub args_tuple_provider_impl: ItemImpl,
}
