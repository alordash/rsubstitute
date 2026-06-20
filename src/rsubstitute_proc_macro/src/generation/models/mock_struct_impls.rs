use syn::*;

pub(crate) struct MockStructImpls {
    pub target_mockable_impl: ItemImpl,
    pub deref_impl: ItemImpl,
    pub deref_mut: ItemImpl,
    pub mock_impl: ItemImpl,
}
