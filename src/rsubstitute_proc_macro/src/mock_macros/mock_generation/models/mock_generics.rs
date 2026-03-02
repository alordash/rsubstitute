use syn::*;

#[derive(Clone)]
pub struct MockGenerics {
    pub source_generics: Generics,
    pub impl_generics: Generics,
}
