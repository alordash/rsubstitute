use syn::*;

pub(crate) struct MockReceivedStruct {
    pub item_struct: ItemStruct,
    pub ty: Type,
    pub clone_for_rsubstitute_trait_impl: ItemImpl,
}
