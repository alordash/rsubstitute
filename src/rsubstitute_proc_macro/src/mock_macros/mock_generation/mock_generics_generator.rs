use syn::Generics;
use crate::mock_macros::mock_generation::models::MockGenerics;

pub trait IMockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics;
}

pub(crate) struct MockGenericsGenerator;

impl IMockGenericsGenerator for MockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics {
        todo!()
    }
}