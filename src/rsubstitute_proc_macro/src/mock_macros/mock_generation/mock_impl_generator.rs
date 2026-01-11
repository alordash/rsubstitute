use crate::constants;
use crate::mock_macros::mock_generation::IMockConstructorBlockGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IMockImplGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> MockImpl;
}

pub(crate) struct MockImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
    pub mock_constructor_block_generator: Arc<dyn IMockConstructorBlockGenerator>,
}

impl IMockImplGenerator for MockImplGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> MockImpl {
        let self_ty = self
            .type_factory
            .create(mock_struct.item_struct.ident.clone());
        let constructor = self.generate_constructor(
            mock_struct,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
        );

        let mut item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items: [constructor].into_iter().collect(),
        };
        self.reference_normalizer.normalize_in_impl(
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            &mut item_impl,
        );
        let mock_impl = MockImpl { item_impl };
        return mock_impl;
    }
}

impl MockImplGenerator {
    const CONSTRUCTOR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));

    fn generate_constructor(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> ImplItem {
        let block = self.mock_constructor_block_generator.generate_for_struct(
            mock_struct,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
        );
        let item_impl = ImplItem::Fn(ImplItemFn {
            attrs: vec![constants::ALLOW_UNUSED_ATTRIBUTE.clone()],
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: Self::CONSTRUCTOR_IDENT.clone(),
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
        });
        return item_impl;
    }
}
