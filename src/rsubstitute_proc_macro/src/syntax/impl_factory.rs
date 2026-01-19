use crate::mock_macros::mock_generation::models::*;
use syn::*;

pub trait IImplFactory {
    fn create_with_default_lifetime(
        &self,
        mock_generics: &MockGenerics,
        self_ty: Type,
        items: Vec<ImplItem>,
    ) -> ItemImpl;
}

pub(crate) struct ImplFactory;

impl IImplFactory for ImplFactory {
    fn create_with_default_lifetime(
        &self,
        mock_generics: &MockGenerics,
        self_ty: Type,
        items: Vec<ImplItem>,
    ) -> ItemImpl {
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: mock_generics.impl_generics.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };
        return item_impl;
    }
}
