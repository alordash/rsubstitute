use proc_macro2::Ident;
use syn::*;

pub(crate) trait IFnOwner {
    fn ident(&self) -> &Ident;
    fn generics(&self) -> &Generics;
}
