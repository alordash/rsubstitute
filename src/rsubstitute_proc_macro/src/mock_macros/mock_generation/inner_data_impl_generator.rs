use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IInnerDataImplGenerator {
    fn generate(&self, inner_data_struct: &InnerDataStruct, new_fn: ImplItemFn) -> InnerDataImpl;
}

pub(crate) struct InnerDataImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl IInnerDataImplGenerator for InnerDataImplGenerator {
    fn generate(&self, inner_data_struct: &InnerDataStruct, new_fn: ImplItemFn) -> InnerDataImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&inner_data_struct.item_struct);
        let item = ImplItem::Fn(new_fn);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: inner_data_struct.item_struct.generics.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items: vec![item],
        };
        let inner_data_impl = InnerDataImpl { item_impl };
        return inner_data_impl;
    }
}
