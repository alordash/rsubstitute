use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::*;
use crate::syntax::*;
use quote::format_ident;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IMockStructDefaultImplGenerator {
    fn generate(
        &self,
        fn_decl: &FnDecl,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        base_caller_struct: &BaseCallerStruct,
        mock_generics: &MockGenerics,
    ) -> MockStructDefaultImpl;
}

pub(crate) struct MockStructDefaultImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub path_factory: Arc<dyn IPathFactory>,
    pub mock_constructor_block_generator: Arc<dyn IMockConstructorBlockGenerator>,
}

impl IMockStructDefaultImplGenerator for MockStructDefaultImplGenerator {
    fn generate(
        &self,
        fn_decl: &FnDecl,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        base_caller_struct: &BaseCallerStruct,
        mock_generics: &MockGenerics,
    ) -> MockStructDefaultImpl {
        let ty = self
            .type_factory
            .create_from_struct(&mock_struct.item_struct);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: mock_generics.impl_generics.clone(),
            trait_: Some((
                None,
                constants::DEFAULT_TRAIT_PATH.clone(),
                Default::default(),
            )),
            self_ty: Box::new(ty),
            brace_token: Default::default(),
            items,
        };
        todo!()
    }
}
