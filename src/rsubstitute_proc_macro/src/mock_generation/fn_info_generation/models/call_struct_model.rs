use syn::*;

pub(crate) struct CallStruct {
    pub generics_info_provider_impl: ItemImpl,
    pub item_struct: ItemStruct,
    pub ty_path: TypePath,
    pub fields_maybe_actual_source_types: Vec<Option<Type>>,
    pub args_infos_provider_trait_impl: ItemImpl,
    pub args_tuple_provider_trait_impl: ItemImpl,
    pub maybe_clone_for_rsubstitute_trait_impl: Option<ItemImpl>
}
