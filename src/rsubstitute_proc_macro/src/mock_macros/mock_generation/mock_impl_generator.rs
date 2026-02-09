use crate::constants;
use crate::mock_macros::mock_generation::IMockConstructorBlockGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IMockImplGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_struct_traits: Vec<&MockStructTrait>,
        maybe_inner_data_param: Option<InnerDataParam>,
    ) -> MockImpl;
}

pub(crate) struct MockImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub mock_constructor_block_generator: Arc<dyn IMockConstructorBlockGenerator>,
}

impl IMockImplGenerator for MockImplGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_struct_traits: Vec<&MockStructTrait>,
        maybe_inner_data_param: Option<InnerDataParam>,
    ) -> MockImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_struct.item_struct);
        let constructor = self.generate_constructor(
            mock_struct,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            mock_struct_traits,
            maybe_inner_data_param,
        );

        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: mock_type.generics.impl_generics.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items: [constructor].into_iter().collect(),
        };
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
        mock_struct_traits: Vec<&MockStructTrait>,
        maybe_inner_data_param: Option<InnerDataParam>,
    ) -> ImplItem {
        let inputs = if let Some(inner_data_param) = &maybe_inner_data_param {
            inner_data_param
                .constructor_arguments
                .iter()
                .map(|constructor_argument| {
                    FnArg::Typed(PatType {
                        attrs: Vec::new(),
                        pat: Box::new(Pat::Ident(PatIdent {
                            attrs: Vec::new(),
                            by_ref: None,
                            mutability: None,
                            ident: constructor_argument.0.clone(),
                            subpat: None,
                        })),
                        colon_token: Default::default(),
                        ty: Box::new(constructor_argument.1.clone()),
                    })
                })
                .collect()
        } else {
            Punctuated::new()
        };
        let block = self.mock_constructor_block_generator.generate(
            mock_struct,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            mock_struct_traits,
            maybe_inner_data_param,
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
                inputs,
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
