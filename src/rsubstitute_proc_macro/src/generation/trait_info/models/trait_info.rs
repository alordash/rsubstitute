use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::preparation::models::*;
use crate::preparation::r#trait::models::*;
use syn::*;

pub(crate) struct TraitInfo {
    pub attributes: Vec<Attribute>,
    pub unsafety: Option<Token![unsafe]>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub source_generics: Generics,
    pub merged_generics: Generics,
    pub constants: Vec<Ordered<TraitItemConstSyntax>>,
    pub assoc_types: Vec<Ordered<TraitItemTypeSyntax>>,
    pub path: Path,
    pub static_fns: Vec<Ordered<FnInfo>>,
    pub associated_fns: Vec<Ordered<FnInfo>>,
    pub mock_struct_ident: Ident,
    pub associated_items_info: AssociatedItemsInfo,
}
