use syn::*;

pub(crate) struct CallStruct {
    pub r#type: Type,
    pub item_struct: ItemStruct,
    pub generics_info_provider_impl: ItemImpl,
    pub call_impl: ItemImpl,
}
