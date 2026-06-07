use syn::*;

pub(crate) struct Argument {
    pub ident: Ident,
    pub inner: PatType,
    pub outer: PatType,
}
