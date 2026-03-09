use syn::*;

#[derive(Clone)]
pub struct MockGenerics {
    pub source_generics: Generics,
    pub impl_generics: Generics,
    pub phantom_fields: Vec<Field>,
}

impl MockGenerics {
    pub fn get_phantom_fields_count(&self) -> usize {
        self.phantom_fields.len()
    }
}
