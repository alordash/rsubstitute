use proc_macro2::Ident;
use syn::ItemStruct;

pub struct ImplementedTraitConfigurator<'a> {
    pub trait_ident: Ident,
    pub item_struct: &'a ItemStruct,
}
