use syn::*;

pub(crate) struct AssociatedGenerics {
    pub trait_item_consts: Vec<TraitItemConst>,
    pub trait_item_types: Vec<TraitItemType>,
    pub generics_params: Vec<GenericParam>, // TODO - remove?
                                     // pub where_predicates: Vec<WherePredicate>,
}
