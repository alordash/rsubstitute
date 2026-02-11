use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub trait IMockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics;
}

pub(crate) struct MockGenericsGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub field_factory: Arc<dyn IFieldFactory>,
    pub generics_merger: Arc<dyn IGenericsMerger>,
}

impl IMockGenericsGenerator for MockGenericsGenerator {
    fn generate(&self, source_generics: &Generics) -> MockGenerics {
        let mut modified_source_generics = source_generics.clone();
        self.add_required_for_lib_type_trait_constraints(&mut modified_source_generics);

        let default_field_lifetime_generic =
            self.generate_default_field_lifetime_generic(source_generics);
        let result_generics = self
            .generics_merger
            .merge(&default_field_lifetime_generic, &modified_source_generics);
        let phantom_type_fields = self.get_all_phantom_fields_from_generics(source_generics);
        let mock_generics = MockGenerics {
            source_generics: source_generics.clone(),
            impl_generics: result_generics,
            phantom_type_fields,
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

    fn generate_default_field_lifetime_generic(&self, source_generics: &Generics) -> Generics {
        let generics = Generics {
            lt_token: Some(Default::default()),
            params: [GenericParam::Lifetime(LifetimeParam {
                attrs: Vec::new(),
                lifetime: constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
                colon_token: Some(Default::default()),
                bounds: source_generics
                    .lifetimes()
                    .map(|lifetime_param| lifetime_param.lifetime.clone())
                    .collect(),
            })]
            .into_iter()
            .collect(),
            gt_token: Some(Default::default()),
            where_clause: None,
        };
        return generics;
    }

    fn get_all_phantom_fields_from_generics(&self, generics: &Generics) -> Vec<Field> {
        let fields = generics
            .params
            .iter()
            .filter_map(|generic_param| match generic_param {
                GenericParam::Lifetime(lifetime_param) => {
                    Some(self.convert_lifetime_param_to_phantom_field(lifetime_param))
                }
                GenericParam::Type(type_param) => {
                    Some(self.convert_type_param_to_phantom_field(type_param))
                }
                _ => None,
            })
            .collect();
        return fields;
    }

    fn convert_lifetime_param_to_phantom_field(&self, lifetime_param: &LifetimeParam) -> Field {
        let ref_ty = Type::Reference(TypeReference {
            and_token: Default::default(),
            lifetime: Some(lifetime_param.lifetime.clone()),
            mutability: None,
            elem: Box::new(constants::VOID_TYPE.clone()),
        });
        let ty = self
            .type_factory
            .wrap_in(ref_ty, constants::PHANTOM_DATA_IDENT.clone());
        let ident = self.format_phantom_field_ident(&lifetime_param.lifetime.ident);
        let field = self.field_factory.create(ident, ty);
        return field;
    }

    fn convert_type_param_to_phantom_field(&self, type_param: &TypeParam) -> Field {
        let ty = self.type_factory.wrap_in(
            self.type_factory.create(type_param.ident.clone()),
            constants::PHANTOM_DATA_IDENT.clone(),
        );
        let ident = self.format_phantom_field_ident(&type_param.ident);
        let field = self.field_factory.create(ident, ty);
        return field;
    }

    fn format_phantom_field_ident(&self, own_ident: &Ident) -> Ident {
        format_ident!("_phantom_{}", own_ident)
    }
}
