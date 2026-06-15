use syn::*;

pub(crate) struct MockReceived {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub clone_impl: ItemImpl,
    pub r#impl: ItemImpl,
}
