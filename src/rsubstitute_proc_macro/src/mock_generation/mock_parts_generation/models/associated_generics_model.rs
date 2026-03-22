use syn::*;

pub(crate) struct AssociatedGenerics {
    pub parent_ident: Ident,
    pub trait_items: Vec<TraitItem>,
    pub generics_params: Vec<GenericParam>,
}
