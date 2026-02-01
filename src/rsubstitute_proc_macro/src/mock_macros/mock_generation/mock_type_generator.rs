use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use std::sync::Arc;
use syn::*;

pub trait IMockTypeGenerator {
    fn generate_for_trait(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType;

    fn generate_for_static(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType;
}

pub(crate) struct MockTypeGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl IMockTypeGenerator for MockTypeGenerator {
    fn generate_for_trait(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType {
        let ty = self.generate_type(
            mock_ident.clone(),
            &mock_generics,
            MockTypeGenericsLifetimeOptions::WithDefaultArgLifetime,
        );
        let mock_type = MockType {
            ident: mock_ident,
            ty,
            generics: mock_generics,
        };
        return mock_type;
    }

    fn generate_for_static(&self, mock_ident: Ident, mock_generics: MockGenerics) -> MockType {
        let ty = self.generate_type(
            mock_ident.clone(),
            &mock_generics,
            MockTypeGenericsLifetimeOptions::WithoutDefaultArgLifetime,
        );
        let mock_type = MockType {
            ident: mock_ident,
            ty,
            generics: mock_generics,
        };
        return mock_type;
    }
}

impl MockTypeGenerator {
    fn generate_type(
        &self,
        mock_ident: Ident,
        mock_generics: &MockGenerics,
        mock_type_generics_lifetime_options: MockTypeGenericsLifetimeOptions,
    ) -> Type {
        let mock_type_generics = match mock_type_generics_lifetime_options {
            MockTypeGenericsLifetimeOptions::WithDefaultArgLifetime => {
                mock_generics.impl_generics.clone()
            }
            MockTypeGenericsLifetimeOptions::WithoutDefaultArgLifetime => {
                let mut new_mock_type_generics = mock_generics.impl_generics.clone();
                new_mock_type_generics.params =
                    new_mock_type_generics.params.into_iter().skip(1).collect();
                new_mock_type_generics
            }
        };
        let ty = self
            .type_factory
            .create_with_generics(mock_ident, mock_type_generics);
        return ty;
    }
}

enum MockTypeGenericsLifetimeOptions {
    WithDefaultArgLifetime,
    WithoutDefaultArgLifetime,
}
