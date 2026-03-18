use syn::*;

pub(crate) struct ArgsCheckerStruct {
    pub generics_info_provider_impl: ItemImpl,
    pub item_struct: ItemStruct,
    pub ty_path: TypePath,
}
