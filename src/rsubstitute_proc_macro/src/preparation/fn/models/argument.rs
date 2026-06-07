use syn::*;

pub(crate) struct Argument {
    pub pat_type: PatType,
    pub ident: Ident,
    pub inner_type: Box<Type>,
}
