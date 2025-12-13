use proc_macro2::Ident;
use syn::{FnArg, ReturnType, Type};

pub struct FnInfo {
    pub(crate) ident: Ident,
    pub(crate) arguments: Vec<FnArg>,
    pub(crate) return_value: Option<Box<Type>>,
}
