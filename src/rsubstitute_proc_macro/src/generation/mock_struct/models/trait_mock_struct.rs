use syn::*;

pub(crate) struct TraitMockStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub trait_impl: ItemImpl,
    pub inner_impl: ItemImpl,
}
