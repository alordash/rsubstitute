use crate::generation::models::*;
use syn::*;

pub(crate) struct Mock {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub maybe_mock_struct_impls: Option<Box<MockStructImpls>>,
}
