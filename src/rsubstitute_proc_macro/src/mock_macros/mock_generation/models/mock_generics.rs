use syn::*;

#[derive(Clone)]
pub struct MockGenerics {
    pub source_generics: Generics,
    pub impl_generics: Generics,
    // TODO - is it really needed?
    pub phantom_type_fields: Vec<Field>,
}

impl MockGenerics {
    pub fn get_phantom_types_count(&self) -> usize {
        self.phantom_type_fields.len()
    }
}
