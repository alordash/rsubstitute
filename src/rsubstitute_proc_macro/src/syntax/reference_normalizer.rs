use crate::constants;
use crate::syntax::IReferenceTypeCrawler;
use std::sync::Arc;
use syn::*;

pub trait IReferenceNormalizer {
    fn staticify_anonymous_lifetimes(&self, ty: &mut Type);

    fn normalize_anonymous_lifetimes_in_struct(&self, item_struct: &mut ItemStruct);
}

pub(crate) struct ReferenceNormalizer {
    pub reference_type_crawler: Arc<dyn IReferenceTypeCrawler>,
}

impl IReferenceNormalizer for ReferenceNormalizer {
    fn staticify_anonymous_lifetimes(&self, ty: &mut Type) {
        let optional_lifetimes: Vec<_> = self.reference_type_crawler.get_all_optional_lifetimes(ty);

        for optional_lifetime in optional_lifetimes {
            if optional_lifetime.is_none() {
                *optional_lifetime = Some(constants::STATIC_LIFETIME.clone());
            }
        }
    }

    fn normalize_anonymous_lifetimes_in_struct(&self, item_struct: &mut ItemStruct) {
        let optional_lifetimes: Vec<_> = item_struct
            .fields
            .iter_mut()
            .flat_map(|field| {
                self.reference_type_crawler
                    .get_all_optional_lifetimes(&mut field.ty)
            })
            .collect();
        for optional_lifetime in optional_lifetimes {
            if optional_lifetime.is_none() {
                *optional_lifetime = Some(constants::DEFAULT_ARG_FIELD_LIFETIME.clone());
            }
        }
    }
}
