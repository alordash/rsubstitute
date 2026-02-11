use crate::constants;
use crate::syntax::IReferenceTypeCrawler;
use std::sync::Arc;
use syn::*;

pub trait IReferenceNormalizer {
    fn staticify_anonymous_lifetimes(&self, ty: &mut Type);

    fn normalize_anonymous_lifetimes_in_struct(&self, item_struct: &mut ItemStruct);

    fn anonymize_fn_arg(&self, fn_arg: &mut FnArg);
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

    fn anonymize_fn_arg(&self, fn_arg: &mut FnArg) {
        let ty = match fn_arg {
            FnArg::Receiver(receiver) => {
                if let Some((_, optional_lifetime)) = &mut receiver.reference {
                    if optional_lifetime.is_none() {
                        *optional_lifetime = Some(constants::ANONYMOUS_LIFETIME.clone());
                    }
                }
                receiver.ty.as_mut()
            }
            FnArg::Typed(pat_type) => pat_type.ty.as_mut(),
        };
        let optional_lifetimes = self.reference_type_crawler.get_all_optional_lifetimes(ty);
        for optional_lifetime in optional_lifetimes {
            if optional_lifetime.is_none() {
                *optional_lifetime = Some(constants::ANONYMOUS_LIFETIME.clone());
            }
        }
    }
}
