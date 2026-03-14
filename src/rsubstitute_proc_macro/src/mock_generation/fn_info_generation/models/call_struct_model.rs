use syn::*;

pub(crate) struct CallStruct {
    pub item_struct: ItemStruct,
    pub ty_path: TypePath,
}
