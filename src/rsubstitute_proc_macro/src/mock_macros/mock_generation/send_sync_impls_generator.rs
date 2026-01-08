use crate::constants;
use crate::mock_macros::mock_generation::models::SendSyncImpls;
use crate::syntax::{IPathFactory, ITypeFactory};
use proc_macro2::Ident;
use std::sync::Arc;
use syn::*;

pub trait ISendSyncImplsGenerator {
    fn generate(&self, item_struct: &ItemStruct) -> SendSyncImpls;
}

pub(crate) struct SendSyncImplsGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl ISendSyncImplsGenerator for SendSyncImplsGenerator {
    fn generate(&self, item_struct: &ItemStruct) -> SendSyncImpls {
        let send_impl = self.generate_item_impl(item_struct, constants::SEND_TRAIT_IDENT.clone());
        let sync_impl = self.generate_item_impl(item_struct, constants::SYNC_TRAIT_IDENT.clone());
        let send_sync_impls = SendSyncImpls {
            send_impl,
            sync_impl,
        };
        return send_sync_impls;
    }
}

impl SendSyncImplsGenerator {
    fn generate_item_impl(&self, item_struct: &ItemStruct, trait_ident: Ident) -> ItemImpl {
        let trait_path = self.path_factory.create(trait_ident);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: Some(Default::default()),
            impl_token: Default::default(),
            generics: item_struct.generics.clone(),
            trait_: Some((None, trait_path, Default::default())),
            self_ty: Box::new(self.type_factory.create_from_struct(&item_struct)),
            brace_token: Default::default(),
            items: Vec::new(),
        };
        return item_impl;
    }
}
