use crate::preparation::models::*;
use crate::preparation::r#fn::models::*;
use crate::preparation::r#trait::models::*;
use syn::*;

pub(crate) struct TraitSyntax {
    pub attributes: Vec<Attribute>,
    pub unsafety: Option<Token![unsafe]>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub source_generics: Generics,
    pub merged_generics: Generics,
    pub constants: Vec<Ordered<TraitItemConstSyntax>>,
    pub assoc_types: Vec<Ordered<TraitItemTypeSyntax>>,
    pub path: Path,
    pub static_fns: Vec<Ordered<FnSyntax>>,
    pub associated_fns: Vec<Ordered<FnSyntax>>,
}
