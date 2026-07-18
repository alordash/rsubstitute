use crate::preparation::models::*;
use crate::preparation::r#fn::models::*;
use syn::*;

pub(crate) struct ImplTraitForStructSyntax {
    pub attributes: Vec<Attribute>,
    pub modules: Vec<Ident>,
    pub target_ident: Ident,
    pub target_type: Type,
    pub trait_ident: Ident,
    pub trait_path: Path,
    pub merged_generics: Generics,
    pub target_simple_generics: Generics,
    pub trait_simple_generics: Generics,
    pub constants: Vec<Ordered<ImplItemConst>>,
    pub static_fns: Vec<Ordered<FnSyntax>>,
    pub associated_fns: Vec<Ordered<FnSyntax>>,
}
