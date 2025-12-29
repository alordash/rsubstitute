use crate::constants;
use crate::syntax::IReferenceTypeCrawler;
use std::sync::Arc;
use syn::*;

pub trait IReferenceNormalizer {
    fn get_normalized_lifetime(&self) -> Lifetime;

    fn normalize_in_struct(&self, item_struct: &mut ItemStruct);

    fn normalize_in_impl(&self, lifetime: Lifetime, item_impl: &mut ItemImpl);
}

pub(crate) struct ReferenceNormalizer {
    pub reference_type_crawler: Arc<dyn IReferenceTypeCrawler>,
}

impl IReferenceNormalizer for ReferenceNormalizer {
    fn get_normalized_lifetime(&self) -> Lifetime {
        constants::DEFAULT_ARG_FIELD_LIFETIME.clone()
    }

    fn normalize_in_struct(&self, item_struct: &mut ItemStruct) {
        let type_references: Vec<_> = item_struct
            .fields
            .iter_mut()
            .flat_map(|field| {
                self.reference_type_crawler
                    .get_all_type_references(&mut field.ty)
            })
            .collect();
        for type_reference in type_references {
            type_reference.lifetime = Some(self.get_normalized_lifetime());
        }
    }

    fn normalize_in_impl(&self, lifetime: Lifetime, item_impl: &mut ItemImpl) {
        if let Type::Path(type_path) = item_impl.self_ty.as_mut() {
            let last_segment = type_path
                .path
                .segments
                .last_mut()
                .expect("impl must have self_ty with non-empty path");
            let generic_argument = GenericArgument::Lifetime(lifetime);
            match &mut last_segment.arguments {
                PathArguments::AngleBracketed(angle_bracketed) => {
                    angle_bracketed.args.push(generic_argument)
                }
                _ => {
                    last_segment.arguments =
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: [generic_argument].into_iter().collect(),
                            gt_token: Default::default(),
                        })
                }
            };
        }
    }
}
