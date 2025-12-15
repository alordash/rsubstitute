use crate::macros::models::FnDecl;
use syn::ItemStruct;

pub struct ArgsMatcherInfo<'a> {
    pub(crate) parent: &'a FnDecl,
    pub(crate) item_struct: ItemStruct,
}
