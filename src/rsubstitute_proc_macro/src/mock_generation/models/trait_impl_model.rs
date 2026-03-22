use syn::*;

pub(crate) struct TraitImpl {
    pub trait_path: Path,
    pub item_impl: ItemImpl,
}

impl TraitImpl {
    pub(crate) fn get_fns(&self) -> Vec<&ImplItemFn> {
        return self
            .item_impl
            .items
            .iter()
            .filter_map(|item| match item {
                ImplItem::Fn(impl_item_fn) => Some(impl_item_fn),
                _ => None,
            })
            .collect();
    }
}
