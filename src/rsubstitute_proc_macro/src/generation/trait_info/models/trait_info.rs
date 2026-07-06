use crate::generation::fn_info::models::*;
use crate::preparation::common::models::*;
use crate::preparation::r#trait::models::*;
use syn::*;

pub(crate) struct TraitInfo {
    pub attributes: Vec<Attribute>,
    pub unsafety: Option<Token![unsafe]>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub merged_generics: Generics,
    pub constants: Vec<Ordered<TraitItemConstSyntax>>,
    pub assoc_types: Vec<Ordered<TraitItemTypeSyntax>>,
    pub path: Path,
    pub methods: Vec<Ordered<FnInfo>>,
}
