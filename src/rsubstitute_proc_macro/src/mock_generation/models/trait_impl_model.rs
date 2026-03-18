use quote::format_ident;
use syn::*;

pub(crate) struct TraitImpl {
    pub trait_path: Path,
    pub item_impl: ItemImpl,
}

impl TraitImpl {
    pub(crate) fn get_trait_ident_from_path(&self) -> Ident {
        let parent_trait_path_idents: Vec<_> = self.trait_path
            .segments
            .iter()
            .map(|x| x.ident.to_string())
            .collect();
        let joined_parent_trait_path_idents = parent_trait_path_idents.join("_");
        return format_ident!("{joined_parent_trait_path_idents}");
    }

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
