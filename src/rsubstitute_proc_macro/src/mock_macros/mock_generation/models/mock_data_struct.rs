use proc_macro2::Ident;
use syn::ItemStruct;

pub struct MockDataStruct {
    pub item_struct: ItemStruct,
    pub field_and_fn_idents: Vec<(Ident, Ident)>
}
