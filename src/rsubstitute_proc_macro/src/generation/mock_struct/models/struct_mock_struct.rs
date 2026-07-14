use syn::*;

pub(crate) struct StructMockStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub item_impl: ItemImpl,
    pub deref_impl: ItemImpl,
    pub deref_mut_impl: ItemImpl,
}
