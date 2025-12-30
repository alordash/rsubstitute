use crate::constants;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::*;
use crate::mock_macros::*;
use proc_macro::TokenStream;
use quote::quote;
use std::sync::Arc;
use syn::*;

pub trait IItemFnHandler {
    fn handle(&self, item_fn: ItemFn) -> TokenStream;
}

pub(crate) struct ItemFnHandler {
    pub fn_decl_extractor: Arc<dyn IFnDeclExtractor>,
    pub mod_generator: Arc<dyn IModGenerator>,
}

impl IItemFnHandler for ItemFnHandler {
    fn handle(&self, mut item_fn: ItemFn) -> TokenStream {
        let _fn_decls = self.fn_decl_extractor.extract_fn(item_fn.clone());
        let target_ident = item_fn.sig.ident.clone();

        let GeneratedMod {
            item_mod,
            use_generated_mod,
        } = self.mod_generator.generate_fn(item_fn.clone());
        let cfg_not_test_attribute = constants::CFG_NOT_TEST_ATTRIBUTE.clone();
        let result = quote! {
            #cfg_not_test_attribute
            #item_fn

            #use_generated_mod
            #item_mod
        };
        return result.into();
    }
}
