use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait IFnSetupGenerator {
    fn generate(
        &self,
        fn_info: &FnInfo,
        mock_struct: &MockStruct,
        mock_setup_struct: &MockSetupStruct,
        base_caller_struct: &BaseCallerStruct,
        mock_generics: &MockGenerics,
    ) -> ItemFn;
}

pub(crate) struct FnSetupGenerator {
    pub input_args_generator: Arc<dyn IInputArgsGenerator>,
    pub setup_output_generator: Arc<dyn ISetupOutputGenerator>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub local_factory: Arc<dyn ILocalFactory>,
    pub path_factory: Arc<dyn IPathFactory>,
    pub get_global_mock_expr_generator: Arc<dyn IGetGlobalMockExprGenerator>,
}

impl IFnSetupGenerator for FnSetupGenerator {
    fn generate(
        &self,
        fn_info: &FnInfo,
        mock_struct: &MockStruct,
        mock_setup_struct: &MockSetupStruct,
        base_caller_struct: &BaseCallerStruct,
        mock_generics: &MockGenerics,
    ) -> ItemFn {
        let output = self.setup_output_generator.generate_for_static(
            fn_info,
            mock_setup_struct,
            base_caller_struct,
        );
        let phantom_types_count = mock_generics.get_phantom_types_count();
        let sig = Signature {
            // TODO - all these `None` should be actually mapped to souarce fns signature
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: constants::MOCK_SETUP_FIELD_IDENT.clone(),
            generics: mock_generics.impl_generics.clone(),
            paren_token: Default::default(),
            inputs: self
                .input_args_generator
                .generate_input_args_with_static_lifetimes(fn_info, phantom_types_count)
                .into_iter()
                .collect(),
            variadic: None,
            output,
        };
        let block = self.generate_fn_setup_block(fn_info, mock_struct, phantom_types_count);
        let item_fn = ItemFn {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            sig,
            block: Box::new(block),
        };
        return item_fn;
    }
}

impl FnSetupGenerator {
    const MOCK_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("mock"));

    fn generate_fn_setup_block(
        &self,
        fn_info: &FnInfo,
        mock_struct: &MockStruct,
        phantom_types_count: usize,
    ) -> Block {
        let mock_var_stmt = Stmt::Local(
            self.local_factory.create(
                Self::MOCK_VAR_IDENT.clone(),
                LocalInit {
                    eq_token: Default::default(),
                    expr: Box::new(
                        self.get_global_mock_expr_generator
                            .generate(&mock_struct.item_struct),
                    ),
                    diverge: None,
                },
            ),
        );
        let mock_var_expr = self.path_factory.create_expr(Self::MOCK_VAR_IDENT.clone());
        let reset_stmt = Stmt::Expr(
            Expr::MethodCall(self.expr_method_call_factory.create_with_base_receiver(
                mock_var_expr.clone(),
                vec![
                    constants::DATA_IDENT.clone(),
                    fn_info.data_field_ident.clone(),
                ],
                constants::RESET_IDENT.clone(),
                Vec::new(),
            )),
            Some(Default::default()),
        );
        let return_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(Expr::MethodCall(
                    self.expr_method_call_factory.create_with_base_receiver(
                        mock_var_expr.clone(),
                        vec![constants::MOCK_SETUP_FIELD_IDENT.clone()],
                        constants::MOCK_SETUP_FIELD_IDENT.clone(),
                        fn_info
                            .args_checker_struct
                            .item_struct
                            .fields
                            .iter()
                            .skip(1 + phantom_types_count)
                            .map(IFieldRequiredIdentGetter::get_required_ident)
                            .collect(),
                    ),
                ))),
            }),
            Some(Default::default()),
        );
        let stmts = vec![mock_var_stmt, reset_stmt, return_stmt];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}
