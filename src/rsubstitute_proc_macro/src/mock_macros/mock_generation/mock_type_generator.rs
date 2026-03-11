use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use std::sync::Arc;

pub(crate) trait IMockTypeGenerator {
    fn generate(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType;

    fn generate_for_struct(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType;
}

pub(crate) struct MockTypeGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl IMockTypeGenerator for MockTypeGenerator {
    fn generate(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType {
        let stores_mock_data = false;
        let result = self.generate_core(mock_ident, mock_generics, stores_mock_data);
        return result;
    }

    fn generate_for_struct(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType {
        let stores_mock_data = true;
        let result = self.generate_core(mock_ident, mock_generics, stores_mock_data);
        return result;
    }
}

impl MockTypeGenerator {
    fn generate_core(
        &self,
        mock_ident: Ident,
        mock_generics: MockGenerics,
        stores_mock_data: bool,
    ) -> MockType {
        let ty = self
            .type_factory
            .create_with_generics(mock_ident.clone(), mock_generics.impl_generics.clone());
        let mock_type = MockType {
            ident: mock_ident,
            ty,
            generics: mock_generics,
            stores_mock_data,
        };
        return mock_type;
    }
}
