use proc_macro2::Ident;
use syn::*;

pub(crate) struct MockDataStruct {
    pub item_struct: ItemStruct,
    pub ty: Type,
    pub field_and_fn_idents: Vec<(Ident, String)>,
}
