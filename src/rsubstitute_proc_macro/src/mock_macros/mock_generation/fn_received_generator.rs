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
        mock_received_struct: &MockReceivedStruct,
        static_mock: &StaticMock,
    ) -> ItemFn;
}

pub(crate) struct FnReceivedGenerator {
    pub received_signature_generator: Arc<dyn IReceivedSignatureGenerator>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
}

impl IFnReceivedGenerator for FnReceivedGenerator {
    fn generate(
        &self,
        fn_info: &FnInfo,
        mock_received_struct: &MockReceivedStruct,
        static_mock: &StaticMock,
    ) -> ItemFn {
        let sig = self
            .received_signature_generator
            .generate_for_static(fn_info, mock_received_struct);
        let block = self.generate_fn_received_block(fn_info, static_mock);
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
    fn generate_fn_received_block(&self, fn_info: &FnInfo, static_mock: &StaticMock) -> Block {
        let static_mock_expr = Expr::MethodCall(self.expr_method_call_factory.create(
            vec![static_mock.item_static.ident.clone()],
            constants::AS_STATIC_METHOD_IDENT.clone(),
            vec![],
        ));
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
                            .skip(1)
                            .map(|field| field.ident.clone().expect("TODO"))
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
