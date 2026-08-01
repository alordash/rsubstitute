use crate::preparation::r#fn::models::*;
use crate::preparation::models::*;
use syn::*;

pub(crate) struct ImplStructSyntax {
    pub attributes: Vec<Attribute>,
    pub target_ident: Ident,
    pub target_path: Path,
    pub target_type: Type,
    pub generics: Generics,
    pub constants: Vec<Ordered<ImplItemConst>>,
    pub static_fns: Vec<Ordered<FnSyntax>>,
    pub associated_fns: Vec<Ordered<FnSyntax>>,
}
