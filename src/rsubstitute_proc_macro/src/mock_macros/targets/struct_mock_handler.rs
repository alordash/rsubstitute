use crate::constants;
use crate::mock_macros::fn_info_generation::IFnInfoGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::StructMockSyntax;
use crate::mock_macros::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::sync::Arc;
use syn::*;

pub trait IStructMockHandler {
    fn handle(&self, struct_mock_syntax: StructMockSyntax) -> TokenStream;
}

struct StructMockHandler {
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

impl IStructMockHandler for StructMockHandler {
    fn handle(&self, struct_mock_syntax: StructMockSyntax) -> TokenStream {
        let mock_ident = format_ident!(
            "{}{}",
            struct_mock_syntax.r#struct.ident,
            constants::MOCK_STRUCT_IDENT_PREFIX
        );
        let fn_decls = self
            .fn_decl_extractor
            .extract_struct_fns(&struct_mock_syntax.struct_fns);
        todo!()
    }
}
