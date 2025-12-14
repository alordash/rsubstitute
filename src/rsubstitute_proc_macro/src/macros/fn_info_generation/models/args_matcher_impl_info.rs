use crate::macros::models::FnInfo;
use syn::ItemImpl;

pub struct ArgsMatcherImplInfo<'a> {
    pub(crate) parent: &'a FnInfo,
    pub(crate) item_impl: ItemImpl,
}
