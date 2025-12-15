use crate::macros::models::FnDecl;
use syn::ItemImpl;

pub struct ArgsMatcherImplInfo<'a> {
    pub(crate) parent: &'a FnDecl,
    pub(crate) item_impl: ItemImpl,
}
