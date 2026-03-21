use syn::*;

pub(crate) struct MockStruct {
    pub item_struct: ItemStruct,
    pub ty: Type,
    pub maybe_clone_for_rsubstitute_trait_impl: Option<ItemImpl>,
}
