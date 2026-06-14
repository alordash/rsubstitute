use syn::*;

pub(crate) struct MockSetup {
    pub r#type: Type,
    pub item_struct: ItemStruct,
    pub clone_impl: ItemImpl,
    pub r#impl: ItemImpl,
}
