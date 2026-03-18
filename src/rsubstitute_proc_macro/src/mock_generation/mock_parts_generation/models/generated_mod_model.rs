use syn::*;

pub(crate) struct GeneratedMod {
    pub item_mod: ItemMod,
    pub use_generated_mod: ItemUse,
}
