use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::input_args_generator::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use std::sync::Arc;
use syn::*;

pub trait IMockSetupImplGenerator {
    fn generate_for_trait(
        &self,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        fn_infos: &[FnInfo],
    ) -> MockSetupImpl;

    fn generate_for_static(
        &self,
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
    pub core_mem_transmute_expr_factory: Arc<dyn ICoreMemTransmuteExprFactory>,
}

impl IMockSetupImplGenerator for MockSetupImplGenerator {
    fn generate_for_trait(
        &self,
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
                let output_type = self.setup_output_generator.generate_for_trait(x);
                return ImplItem::Fn(self.generate_fn_setup(
                    x,
                    use_fn_info_ident_as_method_ident,
                    output_type,
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
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        fn_info: &FnInfo,
    ) -> MockSetupImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_setup_struct.item_struct);
        let use_fn_info_ident_as_method_ident = false;
        let output_type = self.setup_output_generator.generate_for_trait(fn_info);
        let fn_setup = ImplItem::Fn(self.generate_fn_setup(
            fn_info,
            use_fn_info_ident_as_method_ident,
            output_type,
        ));

        let item_impl =
            self.impl_factory
                .create_with_default_lifetime(mock_type, self_ty, vec![fn_setup]);
        let mock_setup_impl = MockSetupImpl { item_impl };
        return mock_setup_impl;
    }
}

impl MockSetupImplGenerator {
    const FN_TUNER_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("fn_tuner"));

    fn generate_fn_setup(
        &self,
        fn_info: &FnInfo,
        use_fn_info_ident_as_method_ident: bool,
        output_type: TypePath,
    ) -> ImplItemFn {
        let block = self.generate_fn_setup_block(fn_info, &output_type);
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: if use_fn_info_ident_as_method_ident {
                fn_info.parent.fn_ident.clone()
            } else {
                constants::MOCK_SETUP_FIELD_IDENT.clone()
            },
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: iter::once(constants::REF_SELF_ARG.clone())
                .chain(self.input_args_generator.generate_input_args(fn_info))
                .collect(),
            variadic: None,
            output: ReturnType::Type(Default::default(), Box::new(Type::Path(output_type))),
        };
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig,
            block,
        };
        return impl_item_fn;
    }

    fn generate_fn_setup_block(&self, fn_info: &FnInfo, output_type: &TypePath) -> Block {
        let (args_checker_var_ident, args_checker_decl_stmt) = self
            .input_args_generator
            .generate_args_checker_var_ident_and_decl_stmt(fn_info);
        let mut fn_tuner_type = output_type.clone();
        let PathArguments::AngleBracketed(ref mut fn_tuner_type_generics) =
            fn_tuner_type.path.segments[0].arguments
        else {
            panic!("Setup function return type (FnTuner) must have generics.")
        };
        let GenericArgument::Lifetime(ref mut fn_tuner_lifetime) = fn_tuner_type_generics.args[0]
        else {
            panic!(
                "Setup function return type (FnTuner) must have lifetime as first generic parameter"
            )
        };
        *fn_tuner_lifetime = Lifetime::new("'_", Span::call_site());
        let fn_tuner_decl_stmt = Stmt::Local(self.local_factory.create_with_type(
            Self::FN_TUNER_VAR_IDENT.clone(),
            Type::Path(fn_tuner_type),
            LocalInit {
                eq_token: Default::default(),
                expr: Box::new(Expr::MethodCall(self.expr_method_call_factory.create(
                    vec![
                        constants::SELF_IDENT.clone(),
                        constants::DATA_IDENT.clone(),
                        fn_info.data_field_ident.clone(),
                    ],
                    constants::FN_DATA_ADD_CONFIG_FN_IDENT.clone(),
                    vec![args_checker_var_ident, constants::SELF_IDENT.clone()],
                ))),
                diverge: None,
            },
        ));
        let return_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                // TODO - make factory for wrapping stuff in Expr::Unsafe
                expr: Some(Box::new(Expr::Unsafe(ExprUnsafe {
                    attrs: Vec::new(),
                    unsafe_token: Default::default(),
                    block: Block {
                        brace_token: Default::default(),
                        stmts: vec![Stmt::Expr(
                            self.core_mem_transmute_expr_factory.create_for_expr(
                                self.path_factory
                                    .create_expr(Self::FN_TUNER_VAR_IDENT.clone()),
                            ),
                            None,
                        )],
                    },
                }))),
            }),
            Some(Default::default()),
        );
        let stmts = vec![args_checker_decl_stmt, fn_tuner_decl_stmt, return_stmt];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}
