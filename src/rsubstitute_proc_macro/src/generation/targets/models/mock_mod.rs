use syn::*;

pub(crate) struct MockMod {
    pub visibility: Visibility,
    pub item_mod: ItemMod,
}
