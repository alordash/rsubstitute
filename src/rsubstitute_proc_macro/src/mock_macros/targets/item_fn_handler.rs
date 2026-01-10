use crate::constants;
use crate::mock_macros::fn_info_generation::IFnInfoGenerator;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::*;
use crate::mock_macros::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::sync::Arc;
use syn::*;

pub trait IItemFnHandler {
    fn handle(&self, item_fn: ItemFn) -> TokenStream;
}

pub(crate) struct ItemFnHandler {
    pub base_fn_generator: Arc<dyn IBaseFnGenerator>,
    pub fn_decl_extractor: Arc<dyn IFnDeclExtractor>,
    pub fn_info_generator: Arc<dyn IFnInfoGenerator>,
    pub base_caller_struct_generator: Arc<dyn IBaseCallerStructGenerator>,
    pub base_caller_impl_generator: Arc<dyn IBaseCallerImplGenerator>,
    pub mock_data_struct_generator: Arc<dyn IMockDataStructGenerator>,
    pub mock_setup_struct_generator: Arc<dyn IMockSetupStructGenerator>,
    pub mock_received_struct_generator: Arc<dyn IMockReceivedStructGenerator>,
    pub mock_struct_generator: Arc<dyn IMockStructGenerator>,
    pub send_sync_impls_generator: Arc<dyn ISendSyncImplsGenerator>,
    pub mock_setup_impl_generator: Arc<dyn IMockSetupImplGenerator>,
    pub mock_received_impl_generator: Arc<dyn IMockReceivedImplGenerator>,
    pub static_mock_generator: Arc<dyn IStaticMockGenerator>,
    pub fn_setup_generator: Arc<dyn IFnSetupGenerator>,
    pub fn_received_generator: Arc<dyn IFnReceivedGenerator>,
    pub mod_generator: Arc<dyn IModGenerator>,
}

impl IItemFnHandler for ItemFnHandler {
    fn handle(&self, item_fn: ItemFn) -> TokenStream {
        let mock_ident = format_ident!(
            "{}{}",
            item_fn.sig.ident,
            constants::MOCK_STRUCT_IDENT_PREFIX
        );
        let base_fn = self.base_fn_generator.generate(item_fn.clone());
        let fn_decl = self.fn_decl_extractor.extract_fn(&item_fn);
        let fn_info = self.fn_info_generator.generate(&fn_decl);
        let base_caller_struct = self.base_caller_struct_generator.generate(&fn_decl);
        let base_caller_impl = self.base_caller_impl_generator.generate(
            &base_caller_struct,
            &fn_info.call_struct,
            &fn_decl,
            &base_fn,
        );
        let fn_infos = [fn_info];
        let mock_data_struct = self
            .mock_data_struct_generator
            .generate_with_non_camel_case_allowed(&mock_ident, &fn_infos);
        let mock_setup_struct = self
            .mock_setup_struct_generator
            .generate_with_non_camel_case_allowed(&mock_ident, &mock_data_struct);
        let mock_received_struct = self
            .mock_received_struct_generator
            .generate_with_non_camel_case_allowed(&mock_ident, &mock_data_struct);
        let mock_struct = self.mock_struct_generator.generate(
            mock_ident.clone(),
            &mock_setup_struct,
            &mock_received_struct,
            &mock_data_struct,
        );
        let send_sync_impls = self
            .send_sync_impls_generator
            .generate(&mock_struct.item_struct);
        let [fn_info] = fn_infos;
        let mock_setup_impl = self.mock_setup_impl_generator.generate_for_static(
            &mock_setup_struct,
            &fn_info,
            &base_caller_struct,
        );
        let mock_received_impl = self
            .mock_received_impl_generator
            .generate_for_static(&mock_received_struct, &fn_info);
        let static_mock = self.static_mock_generator.generate(
            &fn_decl,
            &mock_struct,
            &mock_data_struct,
            &mock_setup_struct,
            &mock_received_struct,
        );
        let fn_setup = self.fn_setup_generator.generate(
            &fn_info,
            &static_mock,
            &mock_setup_struct,
            &base_caller_struct,
        );
        let fn_received =
            self.fn_received_generator
                .generate(&fn_info, &mock_received_struct, &static_mock);

        let generated_mod = self.mod_generator.generate_fn(
            &item_fn,
            base_fn,
            fn_info,
            base_caller_struct,
            base_caller_impl,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            mock_struct,
            send_sync_impls,
            mock_setup_impl,
            mock_received_impl,
            static_mock,
            fn_setup,
            fn_received,
        );
        let GeneratedMod {
            item_mod,
            use_generated_mod,
        } = generated_mod;

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
