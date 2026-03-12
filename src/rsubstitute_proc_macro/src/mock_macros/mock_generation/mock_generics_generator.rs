use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) trait IMockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics;
}

pub(crate) struct MockGenericsGenerator;

impl IMockGenericsGenerator for MockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics {
        let mut modified_source_generics = source_generics.clone();
        modified_source_generics.params.insert(
            0,
            GenericParam::Lifetime(LifetimeParam {
                attrs: Vec::new(),
                lifetime: constants::DEFAULT_ARG_LIFETIME.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
            }),
        );
        let phantom_fields = source_generics
            .params
            .iter()
            .filter_map(|x| self.try_map_generic_param_to_field(x))
            .collect();
        let mock_generics = MockGenerics {
            source_generics: source_generics.clone(),
            impl_generics: modified_source_generics,
            phantom_fields,
        };
        return mock_generics;
    }
}

impl MockGenericsGenerator {
    fn try_map_generic_param_to_field(&self, generic_param: &GenericParam) -> Option<Field> {
        match generic_param {
            GenericParam::Lifetime(generic_lifetime) => Some(field::create(
                self.format_phantom_field_name(&generic_lifetime.lifetime.ident),
                r#type::phantom_data_lifetime(generic_lifetime.lifetime.clone()),
            )),
            GenericParam::Type(generic_type) => Some(field::create(
                self.format_phantom_field_name(&generic_type.ident),
                r#type::phantom_data(generic_type.ident.clone()),
            )),
            _ => None,
        }
    }

    fn format_phantom_field_name(&self, ident: &Ident) -> Ident {
        format_ident!("_phantom_{ident}")
    }
}
