use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::{IExprMethodCallFactory, IPathFactory, IStdMemTransmuteExprFactory};
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait IMockFnBlockGenerator {
    fn generate_for_struct(&self, fn_info: &FnInfo) -> Block;

    fn generate_for_static(&self, fn_info: &FnInfo, static_mock: &StaticMock) -> Block;
}

pub(crate) struct MockFnBlockGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub std_mem_transmute_expr_factory: Arc<dyn IStdMemTransmuteExprFactory>,
}

impl IMockFnBlockGenerator for MockFnBlockGenerator {
    fn generate_for_struct(&self, fn_info: &FnInfo) -> Block {
        let call_stmt = self.generate_call_stmt(fn_info);
        let last_stmt = self.generate_last_stmt(fn_info, ReturnAccessor::SelfRef);
        let stmts = vec![call_stmt, last_stmt];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }

    fn generate_for_static(&self, fn_info: &FnInfo, static_mock: &StaticMock) -> Block {
        let call_stmt = self.generate_call_stmt(fn_info);
        let last_stmt = self.generate_last_stmt(
            fn_info,
            ReturnAccessor::Static(static_mock.item_static.ident.clone()),
        );
        let stmts = vec![call_stmt, last_stmt];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}

impl MockFnBlockGenerator {
    const CALL_VARIABLE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));
    const HANDLE_METHOD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("handle")); // TODO - add test that it equals to FnData::handle
    const HANDLE_RETURNING_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_returning")); // TODO - add test that it equals to FnData::handle_returning

    fn generate_call_stmt(&self, fn_info: &FnInfo) -> Stmt {
        let fn_fields: Vec<_> = fn_info
            .call_struct
            .item_struct
            .fields
            .iter()
            .skip(1)
            .map(|field| {
                let field_ident = field.ident.clone().expect("TODO field ident");
                FieldValue {
                    attrs: Vec::new(),
                    member: Member::Named(field_ident.clone()),
                    colon_token: Some(Default::default()),
                    expr: self.std_mem_transmute_expr_factory.create(field_ident),
                }
            })
            .collect();
        let call_stmt = Stmt::Local(Local {
            attrs: Vec::new(),
            let_token: Default::default(),
            pat: Pat::Path(PatPath {
                attrs: Vec::new(),
                qself: None,
                path: self.path_factory.create(Self::CALL_VARIABLE_IDENT.clone()),
            }),
            init: Some(LocalInit {
                eq_token: Default::default(),
                expr: Box::new(Expr::Unsafe(ExprUnsafe {
                    attrs: Vec::new(),
                    unsafe_token: Default::default(),
                    block: Block {
                        brace_token: Default::default(),
                        stmts: vec![Stmt::Expr(
                            Expr::Struct(ExprStruct {
                                attrs: Vec::new(),
                                qself: None,
                                path: self
                                    .path_factory
                                    .create(fn_info.call_struct.item_struct.ident.clone()),
                                brace_token: Default::default(),
                                fields: std::iter::once(
                                    constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD_VALUE.clone(),
                                )
                                .chain(fn_fields)
                                .collect(),
                                dot2_token: None,
                                rest: None,
                            }),
                            None,
                        )],
                    },
                })),
                diverge: None,
            }),
            semi_token: Default::default(),
        });
        return call_stmt;
    }

    fn generate_last_stmt(&self, fn_info: &FnInfo, return_accessor: ReturnAccessor) -> Stmt {
        let handle_expr = self.generate_handle_expr(fn_info, return_accessor);
        let last_expr = if fn_info.parent.has_return_value() {
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(handle_expr)),
            })
        } else {
            handle_expr
        };
        let last_stmt = Stmt::Expr(last_expr, Some(Default::default()));
        return last_stmt;
    }

    fn generate_handle_expr(&self, fn_info: &FnInfo, return_accessor: ReturnAccessor) -> Expr {
        let return_accessor_ident = match return_accessor {
            ReturnAccessor::SelfRef => constants::SELF_IDENT.clone(),
            ReturnAccessor::Static(static_mock_ident) => static_mock_ident,
        };
        let idents = vec![
            return_accessor_ident,
            constants::DATA_IDENT.clone(),
            fn_info.data_field_ident.clone(),
        ];
        let method = if fn_info.parent.has_return_value() {
            Self::HANDLE_RETURNING_METHOD_IDENT.clone()
        } else {
            Self::HANDLE_METHOD_IDENT.clone()
        };
        let expr_method_call = self.expr_method_call_factory.create(
            idents,
            method,
            vec![Self::CALL_VARIABLE_IDENT.clone()],
        );
        let expr = Expr::MethodCall(expr_method_call);
        return expr;
    }
}

enum ReturnAccessor {
    SelfRef,
    Static(Ident),
}
