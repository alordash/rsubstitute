use syn::*;

pub(crate) struct ArgsCheckerStruct {
    pub item_struct: ItemStruct,
    pub ty_path: TypePath,
}
