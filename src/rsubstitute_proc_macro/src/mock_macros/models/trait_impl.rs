use quote::format_ident;
use syn::*;

pub(crate) struct TraitImpl {
    pub item_impl: ItemImpl,
}

impl TraitImpl {
    pub fn get_trait_ident_from_path(&self) -> Ident {
        let trait_path = &self
            .item_impl
            .trait_
            .as_ref()
            .expect("trait_impls should contain only trait implementations.")
            .1;
        let parent_trait_path_idents: Vec<_> = trait_path
            .segments
            .iter()
            .map(|x| x.ident.to_string())
            .collect();
        let joined_parent_trait_path_idents = parent_trait_path_idents.join("_");
        return format_ident!("{joined_parent_trait_path_idents}");
    }

    pub fn get_fns(&self) -> Vec<&ImplItemFn> {
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
