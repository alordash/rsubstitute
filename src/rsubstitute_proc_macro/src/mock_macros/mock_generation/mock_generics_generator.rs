use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IMockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics;
}

pub(crate) struct MockGenericsGenerator {
    pub generics_merger: Arc<dyn IGenericsMerger>,
}

impl IMockGenericsGenerator for MockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics {
        let mut modified_source_generics = source_generics.clone();
        self.add_required_for_lib_type_trait_constraints(&mut modified_source_generics);

        let result_generics = self.generics_merger.merge(
            &constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC,
            &modified_source_generics,
        );
        let mock_generics = MockGenerics {
            source_generics: source_generics.clone(),
            impl_generics: result_generics,
        };
        return mock_generics;
    }
}

impl MockGenericsGenerator {
    fn add_required_for_lib_type_trait_constraints(&self, generics: &mut Generics) {
        for param in generics.params.iter_mut() {
            match param {
                GenericParam::Type(type_param) => {
                    self.add_if_needed_trait(
                        type_param,
                        constants::DEBUG_TRAIT_IDENT.clone(),
                        constants::DEBUG_TRAIT_PATH.clone(),
                    );
                    self.add_if_needed_trait(
                        type_param,
                        constants::PARTIAL_ORD_TRAIT_IDENT.clone(),
                        constants::PARTIAL_ORD_TRAIT_PATH.clone(),
                    );
                    self.add_if_needed_trait(
                        type_param,
                        constants::CLONE_TRAIT_IDENT.clone(),
                        constants::CLONE_TRAIT_PATH.clone(),
                    );
                }
                _ => (),
            }
        }
    }

    fn add_if_needed_trait(
        &self,
        type_param: &mut TypeParam,
        expected_trait_ident: Ident,
        expected_trait_path: Path,
    ) {
        if !type_param
            .bounds
            .iter()
            .any(|x| self.is_for_trait(x, &expected_trait_ident, &expected_trait_path))
        {
            type_param.bounds.push(TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path: expected_trait_path,
            }))
        }
    }

    fn is_for_trait(
        &self,
        type_param_bound: &TypeParamBound,
        expected_trait_ident: &Ident,
        expected_trait_path: &Path,
    ) -> bool {
        match type_param_bound {
            TypeParamBound::Trait(trait_) => {
                if let Some(trait_ident) = trait_.path.get_ident() {
                    return *trait_ident == *expected_trait_ident;
                }
                if trait_.path.segments.len() != expected_trait_path.segments.len() {
                    return false;
                }
                for (first, second) in trait_
                    .path
                    .segments
                    .iter()
                    .zip(expected_trait_path.segments.iter())
                {
                    match first.arguments {
                        PathArguments::None => {
                            if first.ident != second.ident {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                }
                return true;
            }
            _ => false,
        }
    }
}
