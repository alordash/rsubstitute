use syn::*;

pub(crate) struct StaticReceivedStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub item_impl: ItemImpl,
}
