use crate::constants;
use crate::mock_macros::fn_info_generation::IFnInfoGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
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
    pub mock_generics_generator: Arc<dyn IMockGenericsGenerator>,
    pub fn_info_generator: Arc<dyn IFnInfoGenerator>,
    pub mock_type_generator: Arc<dyn IMockTypeGenerator>,
    pub mock_data_struct_generator: Arc<dyn IMockDataStructGenerator>,
    pub mock_setup_struct_generator: Arc<dyn IMockSetupStructGenerator>,
    pub mock_received_struct_generator: Arc<dyn IMockReceivedStructGenerator>,
    pub mock_struct_generator: Arc<dyn IMockStructGenerator>,
    pub send_sync_impls_generator: Arc<dyn ISendSyncImplsGenerator>,
    pub mock_struct_default_impl_generator: Arc<dyn IMockStructDefaultImplGenerator>,
    pub mock_setup_impl_generator: Arc<dyn IMockSetupImplGenerator>,
    pub mock_received_impl_generator: Arc<dyn IMockReceivedImplGenerator>,
    pub fn_setup_generator: Arc<dyn IFnSetupGenerator>,
    pub fn_received_generator: Arc<dyn IFnReceivedGenerator>,
    pub static_fn_generator: Arc<dyn IStaticFnGenerator>,
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
        let mock_generics = self.mock_generics_generator.generate(&item_fn.sig.generics);
        let mock_type = self
            .mock_type_generator
            .generate(mock_ident.clone(), mock_generics);
        let fn_info = self.fn_info_generator.generate(&fn_decl, &mock_type);
        let fn_infos = [fn_info];
        let mock_data_struct = self
            .mock_data_struct_generator
            .generate_for_static(&mock_type, &fn_infos);
        let mock_setup_struct = self
            .mock_setup_struct_generator
            .generate_with_non_camel_case_allowed(&mock_ident, &mock_type, &mock_data_struct);
        let mock_received_struct = self
            .mock_received_struct_generator
            .generate_with_non_camel_case_allowed(&mock_ident, &mock_type, &mock_data_struct);
        let mock_struct = self.mock_struct_generator.generate_for_static(
            &mock_type,
            &mock_setup_struct,
            &mock_received_struct,
            &mock_data_struct,
        );
        let send_sync_impls = self
            .send_sync_impls_generator
            .generate(&mock_struct.item_struct);
        let mock_struct_default_impl = self.mock_struct_default_impl_generator.generate(
            &mock_struct,
            &mock_data_struct,
            &mock_setup_struct,
            &mock_received_struct,
            &mock_type,
        );
        let [fn_info] = fn_infos;
        let mock_setup_impl = self.mock_setup_impl_generator.generate_for_static(
            &mock_struct,
            &mock_type,
            &mock_setup_struct,
            &fn_info,
        );
        let mock_received_impl = self.mock_received_impl_generator.generate_for_static(
            &mock_type,
            &mock_received_struct,
            &fn_info,
        );
        let fn_setup = self.fn_setup_generator.generate(
            &fn_info,
            &mock_struct,
            &mock_setup_struct,
            &mock_type,
        );
        let fn_received = self.fn_received_generator.generate(
            &fn_info,
            &mock_struct,
            &mock_received_struct,
            &mock_type,
        );
        let static_fn = self
            .static_fn_generator
            .generate(&fn_info, &mock_struct, &mock_type);

        let generated_mod = self.mod_generator.generate_fn(
            &item_fn,
            base_fn,
            fn_info,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            mock_struct,
            send_sync_impls,
            mock_struct_default_impl,
            mock_setup_impl,
            mock_received_impl,
            fn_setup,
            fn_received,
            static_fn,
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
