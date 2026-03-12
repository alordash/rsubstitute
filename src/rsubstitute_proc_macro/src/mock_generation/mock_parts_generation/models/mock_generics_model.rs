use syn::*;

#[derive(Clone)]
pub(crate) struct MockGenerics {
    pub source_generics: Generics,
    pub impl_generics: Generics,
    pub phantom_fields: Vec<Field>,
}

impl MockGenerics {
    pub(crate) fn get_phantom_fields_count(&self) -> usize {
        self.phantom_fields.len()
    }
}
