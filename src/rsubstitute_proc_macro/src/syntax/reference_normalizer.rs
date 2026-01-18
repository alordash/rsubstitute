use crate::constants;
use crate::syntax::IReferenceTypeCrawler;
use std::sync::Arc;
use syn::*;

pub trait IReferenceNormalizer {
    fn get_normalized_lifetime(&self) -> Lifetime;

    fn normalize_in_struct(&self, item_struct: &mut ItemStruct);

    fn staticify(&self, ty: &mut Type);
}

pub(crate) struct ReferenceNormalizer {
    pub reference_type_crawler: Arc<dyn IReferenceTypeCrawler>,
}

impl IReferenceNormalizer for ReferenceNormalizer {
    fn get_normalized_lifetime(&self) -> Lifetime {
        constants::DEFAULT_ARG_FIELD_LIFETIME.clone()
    }

    fn normalize_in_struct(&self, item_struct: &mut ItemStruct) {
        let lifetime_refs: Vec<_> = item_struct
            .fields
            .iter_mut()
            .flat_map(|field| {
                self.reference_type_crawler
                    .get_all_type_references(&mut field.ty)
            })
            .collect();
        for lifetime_ref in lifetime_refs {
            lifetime_ref.set_lifetime(self.get_normalized_lifetime());
        }
    }

    fn staticify(&self, ty: &mut Type) {
        let lifetime_refs: Vec<_> = self.reference_type_crawler.get_all_type_references(ty);

        for lifetime_ref in lifetime_refs {
            lifetime_ref.set_lifetime(constants::STATIC_LIFETIME.clone());
        }
    }
}
