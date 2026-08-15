mod assoc_items_normalization;
mod impl_trait_normalization;
mod lifetimes_in_generic_arguments_normalization;
mod method_normalization;
mod struct_type_references_normalization;

pub(crate) use assoc_items_normalization::*;
pub(crate) use impl_trait_normalization::*;
pub(crate) use lifetimes_in_generic_arguments_normalization::*;
pub(crate) use method_normalization::*;
pub(crate) use struct_type_references_normalization::*;
