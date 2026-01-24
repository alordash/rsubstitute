use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IFnReceivedGenerator {
    fn generate(
        &self,
        fn_info: &FnInfo,
        mock_struct: &MockStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_generics: &MockGenerics,
    ) -> ItemFn;
}

pub(crate) struct FnReceivedGenerator {
    pub received_signature_generator: Arc<dyn IReceivedSignatureGenerator>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub get_global_mock_expr_generator: Arc<dyn IGetGlobalMockExprGenerator>,
}

impl IFnReceivedGenerator for FnReceivedGenerator {
    fn generate(
        &self,
        fn_info: &FnInfo,
        mock_struct: &MockStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_generics: &MockGenerics,
    ) -> ItemFn {
        let sig = self.received_signature_generator.generate_for_static(
            fn_info,
            mock_received_struct,
            mock_generics,
        );
        let block = self.generate_fn_received_block(
            fn_info,
            mock_struct,
            mock_generics.get_phantom_types_count(),
        );
        let item_fn = ItemFn {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            sig,
            block: Box::new(block),
        };
        return item_fn;
    }
}

impl FnReceivedGenerator {
    fn generate_fn_received_block(
        &self,
        fn_info: &FnInfo,
        mock_struct: &MockStruct,
        phantom_types_count: usize,
    ) -> Block {
        let static_mock_expr = self
            .get_global_mock_expr_generator
            .generate(&mock_struct.item_struct);
        let return_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(Expr::MethodCall(
                    self.expr_method_call_factory.create_with_base_receiver(
                        static_mock_expr,
                        vec![constants::MOCK_RECEIVED_FIELD_IDENT.clone()],
                        constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
                        fn_info
                            .args_checker_struct
                            .item_struct
                            .fields
                            .iter()
                            .skip(1 + phantom_types_count)
                            .map(IFieldRequiredIdentGetter::get_required_ident)
                            .chain(std::iter::once(
                                self.received_signature_generator.get_times_arg_ident(),
                            ))
                            .collect(),
                    ),
                ))),
            }),
            Some(Default::default()),
        );
        let stmts = vec![return_stmt];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}
