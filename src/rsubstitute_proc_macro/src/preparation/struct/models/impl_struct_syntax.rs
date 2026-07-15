use crate::preparation::r#fn::models::*;
use crate::preparation::models::*;
use syn::*;

pub(crate) struct ImplStructSyntax {
    pub attributes: Vec<Attribute>,
    pub modules: Vec<Ident>,
    pub target_ident: Ident,
    pub target_type: Type,
    pub generics: Generics,
    pub constants: Vec<Ordered<ImplItemConst>>,
    pub methods: Vec<Ordered<FnSyntax>>,
}
