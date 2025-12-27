use crate::constants;
use crate::syntax::*;
use std::rc::Rc;
use syn::*;

pub trait IImplFactory {
    fn create_with_default_lifetime(&self, self_ty: Type, items: Vec<ImplItem>) -> ItemImpl;
}

pub(crate) struct ImplFactory {
    pub reference_normalizer: Rc<dyn IReferenceNormalizer>,
}

impl IImplFactory for ImplFactory {
    fn create_with_default_lifetime(&self, self_ty: Type, items: Vec<ImplItem>) -> ItemImpl {
        let mut item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };
        self.reference_normalizer.normalize_in_impl(
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            &mut item_impl,
        );
        return item_impl;
    }
}
