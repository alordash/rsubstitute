use syn::*;

pub(crate) struct MockDataStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub mock_data_impl: ItemImpl,
}
