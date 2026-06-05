use syn::*;

pub(crate) struct Argument {
    pub inner: PatType,
    pub outer: PatType
}