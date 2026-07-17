use crate::generation::fn_info::models::*;
use crate::preparation::models::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct ImplTraitForStructInfo {
    pub attributes: Vec<Attribute>,
    pub modules: Vec<Ident>,
    pub target_ident: Ident,
    pub target_type: Type,
    pub trait_path: Path,
    pub generics: Generics,
    pub constants: Vec<Ordered<ImplItemConst>>,
    pub static_fns: Vec<Ordered<FnInfo>>,
    pub associated_fns: Vec<Ordered<FnInfo>>,
}
