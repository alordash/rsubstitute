use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::input_args_generator::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use std::sync::Arc;
use syn::*;

pub trait IMockSetupImplGenerator {
    fn generate(
        &self,
        mock_setup_struct: &MockSetupStruct,
        fn_infos: &[FnInfo],
    ) -> MockSetupImpl;
}

pub(crate) struct MockSetupImplGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub impl_factory: Arc<dyn IImplFactory>,
    pub local_factory: Arc<dyn ILocalFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub input_args_generator: Arc<dyn IInputArgsGenerator>,
    pub fn_setup_output_generator: Arc<dyn IFnSetupOutputGenerator>,
}

impl IMockSetupImplGenerator for MockSetupImplGenerator {
    fn generate(
        &self,
        mock_setup_struct: &MockSetupStruct,
        fn_infos: &[FnInfo],
    ) -> MockSetupImpl {
        let self_ty = self
            .type_factory
            .create(mock_setup_struct.item_struct.ident.clone());
        let fn_setups = fn_infos
            .iter()
            .map(|x| ImplItem::Fn(self.generate_fn_setup(x)))
            .collect();

        let item_impl = self
            .impl_factory
            .create_with_default_lifetime(self_ty, fn_setups);
        let mock_setup_impl = MockSetupImpl { item_impl };
        return mock_setup_impl;
    }
}

impl MockSetupImplGenerator {
    const FN_CONFIG_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("fn_config"));
    const SHARED_FN_CONFIG_VAR_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("shared_fn_config"));

    fn generate_fn_setup(&self, fn_info: &FnInfo) -> ImplItemFn {
        let output = self.fn_setup_output_generator.generate_for_struct(fn_info);
        let sig = Signature {
            // TODO - all these `None` should be actually mapped to souarce fns signature
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_info.parent.ident.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: iter::once(constants::REF_SELF_ARG_WITH_LIFETIME.clone())
                .chain(self.input_args_generator.generate_input_args(fn_info))
                .collect(),
            variadic: None,
            output,
        };
        let block = self.generate_fn_setup_block(fn_info);
        let impl_item_fn = ImplItemFn {
            attrs: vec![
                constants::ALLOW_UNUSED_ATTRIBUTE.clone(),
                constants::ALLOW_ELIDED_NAMED_LIFETIMES_ATTRIBUTE.clone(),
            ],
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig,
            block,
        };
        return impl_item_fn;
    }

    fn generate_fn_setup_block(&self, fn_info: &FnInfo) -> Block {
        let (args_checker_var_ident, args_checker_decl_stmt) = self
            .input_args_generator
            .generate_args_checker_var_ident_and_decl_stmt(fn_info);
        let fn_config_decl_stmt = Stmt::Local(self.local_factory.create(
            Self::FN_CONFIG_VAR_IDENT.clone(),
            LocalInit {
                eq_token: Default::default(),
                expr: Box::new(Expr::MethodCall(self.expr_method_call_factory.create(
                    vec![
                        constants::SELF_IDENT.clone(),
                        constants::DATA_IDENT.clone(),
                        fn_info.data_field_ident.clone(),
                    ],
                    constants::FN_DATA_ADD_CONFIG_FN_IDENT.clone(),
                    vec![args_checker_var_ident],
                ))),
                diverge: None,
            },
        ));
        let shared_fn_config_decl_stmt = Stmt::Local(
            self.local_factory.create(
                Self::SHARED_FN_CONFIG_VAR_IDENT.clone(),
                LocalInit {
                    eq_token: Default::default(),
                    expr: Box::new(Expr::Call(ExprCall {
                        attrs: Vec::new(),
                        func: Box::new(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: constants::SHARED_FN_CONFIG_TYPE_IDENT.clone(),
                                        arguments: PathArguments::None,
                                    },
                                    PathSegment {
                                        ident: constants::SHARED_FN_CONFIG_NEW_FN_IDENT.clone(),
                                        arguments: PathArguments::None,
                                    },
                                ]
                                .into_iter()
                                .collect(),
                            },
                        })),
                        paren_token: Default::default(),
                        // TODO - add factory for ExprPath
                        args: [
                            Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: self.path_factory.create(Self::FN_CONFIG_VAR_IDENT.clone()),
                            }),
                            Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: constants::SELF_IDENT_PATH.clone(),
                            }),
                            Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: constants::OPTION_NONE_PATH.clone(),
                            }),
                        ]
                        .into_iter()
                        .collect(),
                    })),
                    diverge: None,
                },
            ),
        );
        let return_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: self
                        .path_factory
                        .create(Self::SHARED_FN_CONFIG_VAR_IDENT.clone()),
                }))),
            }),
            Some(Default::default()),
        );
        let stmts = vec![
            args_checker_decl_stmt,
            fn_config_decl_stmt,
            shared_fn_config_decl_stmt,
            return_stmt,
        ];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}
