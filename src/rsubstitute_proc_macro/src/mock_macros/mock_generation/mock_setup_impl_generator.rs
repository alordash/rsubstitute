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
    fn generate_for_trait(
        &self,
        mock_struct: &MockStruct,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        fn_infos: &[FnInfo],
    ) -> MockSetupImpl;

    fn generate_for_static(
        &self,
        mock_struct: &MockStruct,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        fn_info: &FnInfo,
    ) -> MockSetupImpl;
}

pub(crate) struct MockSetupImplGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub impl_factory: Arc<dyn IImplFactory>,
    pub local_factory: Arc<dyn ILocalFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub input_args_generator: Arc<dyn IInputArgsGenerator>,
    pub setup_output_generator: Arc<dyn ISetupOutputGenerator>,
}

impl IMockSetupImplGenerator for MockSetupImplGenerator {
    fn generate_for_trait(
        &self,
        mock_struct: &MockStruct,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        fn_infos: &[FnInfo],
    ) -> MockSetupImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_setup_struct.item_struct);
        let use_fn_info_ident_as_method_ident = true;
        let fn_setups = fn_infos
            .iter()
            .map(|x| {
                let output = self
                    .setup_output_generator
                    .generate_for_trait(x, mock_struct);
                return ImplItem::Fn(self.generate_fn_setup(
                    x,
                    use_fn_info_ident_as_method_ident,
                    output,
                    mock_type.generics.get_phantom_types_count(),
                ));
            })
            .collect();

        let item_impl = self
            .impl_factory
            .create_with_default_lifetime(mock_type, self_ty, fn_setups);
        let mock_setup_impl = MockSetupImpl { item_impl };
        return mock_setup_impl;
    }

    fn generate_for_static(
        &self,
        mock_struct: &MockStruct,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        fn_info: &FnInfo,
    ) -> MockSetupImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_setup_struct.item_struct);
        let use_fn_info_ident_as_method_ident = false;
        let output = self
            .setup_output_generator
            .generate_for_trait(fn_info, mock_struct);
        let fn_setup = ImplItem::Fn(self.generate_fn_setup(
            fn_info,
            use_fn_info_ident_as_method_ident,
            output,
            mock_type.generics.get_phantom_types_count(),
        ));

        let item_impl =
            self.impl_factory
                .create_with_default_lifetime(mock_type, self_ty, vec![fn_setup]);
        let mock_setup_impl = MockSetupImpl { item_impl };
        return mock_setup_impl;
    }
}

impl MockSetupImplGenerator {
    const FN_CONFIG_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("fn_config"));
    const SHARED_FN_CONFIG_VAR_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("shared_fn_config"));

    fn generate_fn_setup(
        &self,
        fn_info: &FnInfo,
        use_fn_info_ident_as_method_ident: bool,
        output: ReturnType,
        phantom_types_count: usize,
    ) -> ImplItemFn {
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: if use_fn_info_ident_as_method_ident {
                fn_info.parent.raw_fn_ident.clone()
            } else {
                constants::MOCK_SETUP_FIELD_IDENT.clone()
            },
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: iter::once(constants::REF_SELF_ARG_WITH_LIFETIME.clone())
                .chain(
                    self.input_args_generator
                        .generate_input_args(fn_info, phantom_types_count),
                )
                .collect(),
            variadic: None,
            output,
        };
        let block = self.generate_fn_setup_block(fn_info);
        let impl_item_fn = ImplItemFn {
            attrs: vec![
                constants::ALLOW_UNUSED_ATTRIBUTE.clone(),
                constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone(),
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
                        func: Box::new(self.path_factory.create_expr_from_parts(vec![
                            constants::SHARED_FN_CONFIG_TYPE_IDENT.clone(),
                            constants::SHARED_FN_CONFIG_NEW_FN_IDENT.clone(),
                        ])),
                        paren_token: Default::default(),
                        args: [
                            self.path_factory
                                .create_expr(Self::FN_CONFIG_VAR_IDENT.clone()),
                            self.path_factory.create_expr(constants::SELF_IDENT.clone()),
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
                expr: Some(Box::new(
                    self.path_factory
                        .create_expr(Self::SHARED_FN_CONFIG_VAR_IDENT.clone()),
                )),
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
