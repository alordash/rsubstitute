use syn::*;

pub(crate) struct MockSetupStruct {
    pub item_struct: ItemStruct,
    pub ty: Type,
    pub clone_for_rsubstitute_trait_impl: ItemImpl
}
