use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait IMockFnBlockGenerator {
    fn generate_for_trait(&self, fn_info: &FnInfo) -> Block;

    fn generate_for_static(&self, fn_info: &FnInfo, mock_type: &MockType) -> Block;
}

pub(crate) struct MockFnBlockGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub std_mem_transmute_expr_factory: Arc<dyn IStdMemTransmuteExprFactory>,
    pub field_value_factory: Arc<dyn IFieldValueFactory>,
    pub get_global_mock_expr_generator: Arc<dyn IGetGlobalMockExprGenerator>,
    pub field_checker: Arc<dyn IFieldChecker>,
    pub local_factory: Arc<dyn ILocalFactory>,
}

impl IMockFnBlockGenerator for MockFnBlockGenerator {
    fn generate_for_trait(&self, fn_info: &FnInfo) -> Block {
        let call_stmt = self.generate_call_stmt(fn_info);
        let last_stmts = self.generate_last_stmts(fn_info, ReturnAccessor::SelfRef);
        let stmts = [call_stmt].into_iter().chain(last_stmts).collect();
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }

    fn generate_for_static(&self, fn_info: &FnInfo, mock_type: &MockType) -> Block {
        let call_stmt = self.generate_call_stmt(fn_info);
        let last_stmts = self.generate_last_stmts(fn_info, ReturnAccessor::Static(mock_type));
        let stmts = [call_stmt].into_iter().chain(last_stmts).collect();
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}

impl MockFnBlockGenerator {
    const CALL_VARIABLE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));
    const HANDLE_METHOD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("handle"));
    const HANDLE_RETURNING_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_returning"));
    const HANDLE_RETURNING_MUT_REF_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_returning_mut_ref"));
    const HANDLE_BASE_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_base"));
    const HANDLE_BASE_RETURNING_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_base_returning"));
    const HANDLE_BASE_RETURNING_MUT_REF_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_base_returning_mut_ref"));
    const MOCK_VARIABLE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("mock"));

    fn generate_call_stmt(&self, fn_info: &FnInfo) -> Stmt {
        let field_values: Vec<_> = fn_info
            .call_struct
            .item_struct
            .fields
            .iter()
            .map(|field| {
                let field_ident = field.get_required_ident();
                if self.field_checker.is_phantom_data(field) {
                    return self.field_value_factory.create_as_phantom_data(field_ident);
                }
                let field_value = FieldValue {
                    attrs: Vec::new(),
                    member: Member::Named(field_ident.clone()),
                    colon_token: Some(Default::default()),
                    expr: self.std_mem_transmute_expr_factory.create(field_ident),
                };
                return field_value;
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
                                fields: field_values.into_iter().collect(),
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

    fn generate_last_stmts(&self, fn_info: &FnInfo, return_accessor: ReturnAccessor) -> Vec<Stmt> {
        let base_receiver = self.path_factory.create_expr(match return_accessor {
            ReturnAccessor::SelfRef => constants::SELF_IDENT.clone(),
            ReturnAccessor::Static(_) => Self::MOCK_VARIABLE_IDENT.clone(),
        });
        let mut handle_expr = self.generate_handle_expr(fn_info, base_receiver);
        if fn_info.parent.has_return_value() {
            handle_expr = Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(handle_expr)),
            });
        }
        let handle_stmt = Stmt::Expr(handle_expr, Some(Default::default()));
        let last_stmts = match return_accessor {
            ReturnAccessor::SelfRef => vec![handle_stmt],
            ReturnAccessor::Static(mock_type) => {
                let mock_var_stmt = Stmt::Local(
                    self.local_factory.create(
                        Self::MOCK_VARIABLE_IDENT.clone(),
                        LocalInit {
                            eq_token: Default::default(),
                            expr: Box::new(
                                self.get_global_mock_expr_generator
                                    .generate(mock_type.ty.clone()),
                            ),
                            diverge: None,
                        },
                    ),
                );
                vec![mock_var_stmt, handle_stmt]
            }
        };
        return last_stmts;
    }

    fn generate_handle_expr(&self, fn_info: &FnInfo, base_receiver: Expr) -> Expr {
        let idents = vec![
            constants::DATA_IDENT.clone(),
            fn_info.data_field_ident.clone(),
        ];
        let return_type_is_mut_ref = match fn_info.parent.get_return_value_type() {
            Type::Reference(type_reference) if type_reference.mutability.is_some() => true,
            _ => false,
        };
        let method_name = match (
            fn_info.maybe_base_caller_impl.is_some(),
            fn_info.parent.has_return_value(),
            return_type_is_mut_ref,
        ) {
            (false, false, _) => Self::HANDLE_METHOD_IDENT.clone(),
            (false, true, false) => Self::HANDLE_RETURNING_METHOD_IDENT.clone(),
            (false, true, true) => Self::HANDLE_RETURNING_MUT_REF_METHOD_IDENT.clone(),
            (true, false, _) => Self::HANDLE_BASE_METHOD_IDENT.clone(),
            (true, true, false) => Self::HANDLE_BASE_RETURNING_METHOD_IDENT.clone(),
            (true, true, true) => Self::HANDLE_BASE_RETURNING_MUT_REF_METHOD_IDENT.clone(),
        };
        let args = if fn_info.maybe_base_caller_impl.is_some() {
            vec![
                Expr::Reference(ExprReference {
                    attrs: Vec::new(),
                    and_token: Default::default(),
                    mutability: None,
                    expr: Box::new(base_receiver.clone()),
                }),
                self.path_factory
                    .create_expr(Self::CALL_VARIABLE_IDENT.clone()),
            ]
        } else {
            vec![
                self.path_factory
                    .create_expr(Self::CALL_VARIABLE_IDENT.clone()),
            ]
        };
        let expr_method_call = self
            .expr_method_call_factory
            .create_with_base_receiver_and_expr_args(base_receiver, idents, method_name, args);
        let expr = Expr::MethodCall(expr_method_call);
        return expr;
    }
}

#[derive(Clone, Copy)]
enum ReturnAccessor<'a> {
    SelfRef,
    Static(&'a MockType),
}
