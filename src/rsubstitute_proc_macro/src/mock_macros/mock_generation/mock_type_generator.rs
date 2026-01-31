use crate::mock_macros::mock_generation::models::*;
use proc_macro2::Ident;

pub trait IMockTypeGenerator {
    fn generate(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType;
}

pub(crate) struct MockTypeGenerator;

impl IMockTypeGenerator for MockTypeGenerator {
    fn generate(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType {
        let mock_type = MockType {
            ident: mock_ident,
            generics: mock_generics,
        };
        return mock_type;
    }
}
