use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use std::sync::Arc;

pub trait IMockTypeGenerator {
    fn generate(&self, mock_ident: Ident, mock_generics: &MockGenerics) -> MockType;
}

pub(crate) struct MockTypeGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl IMockTypeGenerator for MockTypeGenerator {
    fn generate(&self, mock_ident: Ident, mock_generics: &MockGenerics) -> MockType {
        let ty = self
            .type_factory
            .create_with_generics(mock_ident, mock_generics.impl_generics.clone());
        let mock_type = MockType { ty };
        return mock_type;
    }
}
