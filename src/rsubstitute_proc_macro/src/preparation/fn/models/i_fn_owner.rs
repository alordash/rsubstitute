use proc_macro2::Ident;
use syn::*;

pub(crate) trait IFnOwner {
    fn maybe_ident(&self) -> Option<&Ident>;
    fn generics(&self) -> &Generics;
}
