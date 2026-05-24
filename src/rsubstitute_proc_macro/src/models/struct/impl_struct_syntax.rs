use crate::models::r#fn::FnSyntax;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct ImplStructSyntax {
    pub attributes: Vec<Attribute>,
    pub target_ident: Ident,
    pub target_type: Type,
    pub generics: Generics,
    pub constants: Vec<ImplItemConst>,
    pub assoc_types: Vec<ImplItemType>,
    pub methods: Vec<FnSyntax>,
}
