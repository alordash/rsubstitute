use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::Generics;

pub trait IMockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics;
}

pub(crate) struct MockGenericsGenerator {
    pub generics_merger: Arc<dyn IGenericsMerger>,
}

impl IMockGenericsGenerator for MockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics {
        let result_generics = self.generics_merger.merge(
            &constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC,
            source_generics,
        );
        let mock_generics = MockGenerics {
            generics: result_generics,
        };
        return mock_generics;
    }
}
