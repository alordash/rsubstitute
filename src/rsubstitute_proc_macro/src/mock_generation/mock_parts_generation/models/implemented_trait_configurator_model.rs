use proc_macro2::Ident;
use syn::ItemStruct;

pub(crate) struct ImplementedTraitConfigurator<'a> {
    pub(crate) trait_ident: Ident,
    pub item_struct: &'a ItemStruct,
}
