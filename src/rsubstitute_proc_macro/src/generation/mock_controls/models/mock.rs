use syn::*;

pub(crate) struct Mock {
    pub path: Path,
    pub item_struct: ItemStruct,
}
