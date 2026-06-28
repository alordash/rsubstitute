use syn::*;

pub(crate) struct MockStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
}
