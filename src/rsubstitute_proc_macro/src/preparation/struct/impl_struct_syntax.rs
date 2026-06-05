use crate::preparation::r#fn::FnSyntax;
use syn::*;

pub(crate) struct ImplStructSyntax {
    pub attributes: Vec<Attribute>,
    pub modules: Vec<Ident>,
    pub target_ident: Ident,
    pub target_type: Type,
    pub generics: Generics,
    pub constants: Vec<ImplItemConst>,
    pub assoc_types: Vec<ImplItemType>,
    pub methods: Vec<FnSyntax>,
}
