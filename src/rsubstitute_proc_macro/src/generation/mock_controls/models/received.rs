use syn::*;

pub(crate) struct ReceivedStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub item_impl: ItemImpl,
}