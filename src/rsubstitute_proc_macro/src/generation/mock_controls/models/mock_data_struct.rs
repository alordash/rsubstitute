use syn::*;

pub(crate) struct MockDataStruct {
    pub r#type: Type,
    pub item_struct: ItemStruct,
    pub mock_data_impl: ItemImpl
}
