use syn::*;

pub(crate) struct MockModUsages {
    pub use_rsubstitute_for_generated: ItemUse,
    pub use_super: ItemUse,
}
