use crate::constants;
use crate::lifetime_ref::LifetimeRef;
use crate::syntax::IReferenceTypeCrawler;
use std::sync::Arc;
use syn::*;

pub trait IReferenceNormalizer {
    fn get_normalized_lifetime(&self) -> Lifetime;

    fn staticify(&self, ty: &mut Type);

    fn anonymize_fn_arg(&self, fn_arg: &mut FnArg);
}

pub(crate) struct ReferenceNormalizer {
    pub reference_type_crawler: Arc<dyn IReferenceTypeCrawler>,
}

impl IReferenceNormalizer for ReferenceNormalizer {
    fn get_normalized_lifetime(&self) -> Lifetime {
        constants::DEFAULT_ARG_FIELD_LIFETIME.clone()
    }

    fn staticify(&self, ty: &mut Type) {
        let lifetime_refs: Vec<_> = self.reference_type_crawler.get_all_type_references(ty);

        for lifetime_ref in lifetime_refs {
            lifetime_ref.set_lifetime(constants::STATIC_LIFETIME.clone());
        }
    }

    fn anonymize_fn_arg(&self, fn_arg: &mut FnArg) {
        let ty = match fn_arg {
            FnArg::Receiver(receiver) => {
                if let Some((_, lifetime)) = &mut receiver.reference {
                    self.anonymize_input_reference_lifetime(LifetimeRef::Optional(lifetime));
                }
                receiver.ty.as_mut()
            }
            FnArg::Typed(pat_type) => pat_type.ty.as_mut(),
        };
        let lifetime_refs = self.reference_type_crawler.get_all_type_references(ty);
        for lifetime_ref in lifetime_refs {
            self.anonymize_input_reference_lifetime(lifetime_ref);
        }
    }
}

impl ReferenceNormalizer {
    fn anonymize_input_reference_lifetime(&self, lifetime_ref: LifetimeRef) {
        if let LifetimeRef::Optional(optional_lifetime) = lifetime_ref
            && optional_lifetime.is_none()
        {
            *optional_lifetime = Some(constants::ANONYMOUS_LIFETIME.clone());
        }
    }
}
