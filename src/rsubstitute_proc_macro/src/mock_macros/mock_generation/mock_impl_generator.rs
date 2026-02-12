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
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub std_mem_transmute_expr_factory: Arc<dyn IStdMemTransmuteExprFactory>,
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
        let setup_fn = self.generate_setup_fn(&mock_setup_struct);
        let received_fn = self.generate_received_fn(&mock_received_struct);

        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: mock_type.generics.impl_generics.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items: [constructor, setup_fn, received_fn].into_iter().collect(),
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
        let item_impl_fn = ImplItemFn {
            attrs: Vec::new(),
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
        };
        return ImplItem::Fn(item_impl_fn);
    }

    fn generate_setup_fn(&self, mock_setup_struct: &MockSetupStruct) -> ImplItem {
        let impl_item = self.generate_config_fn(
            mock_setup_struct.item_struct.ident.clone(),
            mock_setup_struct.item_struct.generics.clone(),
            ConfigMember::Setup,
        );
        return impl_item;
    }

    fn generate_received_fn(&self, mock_received_struct: &MockReceivedStruct) -> ImplItem {
        let impl_item = self.generate_config_fn(
            mock_received_struct.item_struct.ident.clone(),
            mock_received_struct.item_struct.generics.clone(),
            ConfigMember::Received,
        );
        return impl_item;
    }

    fn generate_config_fn(
        &self,
        config_ident: Ident,
        mut config_generics: Generics,
        config_member: ConfigMember,
    ) -> ImplItem {
        for lifetime_param in config_generics.lifetimes_mut() {
            lifetime_param.lifetime.ident = constants::CONFIG_LIFETIME_IDENT.clone();
        }
        let config_type = self
            .type_factory
            .create_with_generics(config_ident, config_generics);
        let ident = match config_member {
            ConfigMember::Setup => constants::MOCK_SETUP_FIELD_IDENT.clone(),
            ConfigMember::Received => constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
        };
        let block = self.generate_config_block(ident.clone());
        let item_impl_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident,
                generics: constants::CONFIG_LIFETIME_GENERICS.clone(),
                paren_token: Default::default(),
                inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
                variadic: None,
                output: ReturnType::Type(Default::default(), Box::new(config_type)),
            },
            block,
        };
        return ImplItem::Fn(item_impl_fn);
    }

    fn generate_config_block(&self, config_ident: Ident) -> Block {
        let config_clone_expr = Expr::MethodCall(self.expr_method_call_factory.create(
            vec![constants::SELF_IDENT.clone(), config_ident],
            constants::CLONE_FN_IDENT.clone(),
            Vec::new(),
        ));
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![Stmt::Expr(
                Expr::Unsafe(ExprUnsafe {
                    attrs: Vec::new(),
                    unsafe_token: Default::default(),
                    block: Block {
                        brace_token: Default::default(),
                        stmts: vec![Stmt::Expr(
                            self.std_mem_transmute_expr_factory
                                .create_for_expr(config_clone_expr),
                            None,
                        )],
                    },
                }),
                None,
            )],
        };
        return block;
    }
}

enum ConfigMember {
    Setup,
    Received,
}
