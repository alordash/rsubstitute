use crate::models::r#fn::FnSyntax;
use syn::*;

pub(crate) struct TraitSyntax {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub merged_generics: Generics,
    pub constants: Vec<TraitItemConst>,
    pub assoc_types: Vec<TraitItemType>,
    pub methods: Vec<FnSyntax>,
}
