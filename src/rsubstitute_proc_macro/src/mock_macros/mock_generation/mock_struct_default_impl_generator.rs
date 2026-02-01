use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IMockStructDefaultImplGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_type: &MockType,
    ) -> MockStructDefaultImpl;
}

pub(crate) struct MockStructDefaultImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub mock_constructor_block_generator: Arc<dyn IMockConstructorBlockGenerator>,
}

impl IMockStructDefaultImplGenerator for MockStructDefaultImplGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_type: &MockType,
    ) -> MockStructDefaultImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_struct.item_struct);
        let block = self.mock_constructor_block_generator.generate(
            mock_struct,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
        );
        let default_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: constants::DEFAULT_FN_IDENT.clone(),
                generics: Generics::default(),
                paren_token: Default::default(),
                inputs: Punctuated::new(),
                variadic: None,
                output: ReturnType::Type(
                    Default::default(),
                    Box::new(constants::SELF_TYPE.clone()),
                ),
            },
            block,
        };
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: mock_type.generics.impl_generics.clone(),
            trait_: Some((
                None,
                constants::DEFAULT_TRAIT_PATH.clone(),
                Default::default(),
            )),
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items: vec![ImplItem::Fn(default_fn)],
        };
        let mock_struct_default_impl = MockStructDefaultImpl { item_impl };
        return mock_struct_default_impl;
    }
}
