use proc_macro2::Ident;
use syn::{FnArg, Type};

pub struct FnDecl {
    pub(crate) ident: Ident,
    pub(crate) arguments: Vec<FnArg>,
    pub(crate) return_value: Option<Box<Type>>,
}
