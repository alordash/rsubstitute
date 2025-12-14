use crate::macros::models::FnInfo;
use syn::ItemStruct;

pub struct CallInfo<'a> {
    pub(crate) parent: &'a FnInfo,
    pub(crate) item_struct: ItemStruct,
}
