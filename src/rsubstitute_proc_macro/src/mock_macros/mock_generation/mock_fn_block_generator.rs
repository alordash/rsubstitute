use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub(crate) trait IMockFnBlockGenerator {
    fn generate_for_trait(&self, fn_info: &FnInfo) -> Block;

    fn generate_for_struct_trait_fn(&self, fn_info: &FnInfo, trait_ident: &Ident) -> Block;

    fn generate_for_static(&self, fn_info: &FnInfo, mock_type: &MockType) -> Block;
}

pub(crate) struct MockFnBlockGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub expr_reference_factory: Arc<dyn IExprReferenceFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub transmute_lifetime_expr_factory: Arc<dyn ITransmuteLifetimeExprFactory>,
    pub field_value_factory: Arc<dyn IFieldValueFactory>,
    pub get_global_mock_expr_generator: Arc<dyn IGetGlobalMockExprGenerator>,
    pub field_checker: Arc<dyn IFieldChecker>,
    pub local_factory: Arc<dyn ILocalFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub base_fn_ident_formatter: Arc<dyn IBaseFnIdentFormatter>,
}

impl IMockFnBlockGenerator for MockFnBlockGenerator {
    fn generate_for_trait(&self, fn_info: &FnInfo) -> Block {
        let call_stmt = self.generate_call_stmt(fn_info);
        let last_stmts = self.generate_last_stmts(fn_info, Target::Other, None);
        let stmts = [call_stmt].into_iter().chain(last_stmts).collect();
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }

    fn generate_for_struct_trait_fn(&self, fn_info: &FnInfo, trait_ident: &Ident) -> Block {
        let call_stmt = self.generate_call_stmt(fn_info);
        let last_stmts = self.generate_last_stmts(fn_info, Target::Other, Some(trait_ident));
        let stmts = [call_stmt].into_iter().chain(last_stmts).collect();
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }

    fn generate_for_static(&self, fn_info: &FnInfo, mock_type: &MockType) -> Block {
        let call_stmt = self.generate_call_stmt(fn_info);
        let last_stmts = self.generate_last_stmts(fn_info, Target::StaticFn(mock_type), None);
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
    const HANDLE_BASE_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_base"));
    const HANDLE_BASE_RETURNING_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_base_returning"));
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
                    expr: self.transmute_lifetime_expr_factory.create(field_ident),
                };
                return field_value;
            })
            .collect();
        let mut call_struct_type_generics = fn_info.call_struct.item_struct.generics.clone();
        let Some(GenericParam::Lifetime(first_lifetime_param)) =
            call_struct_type_generics.params.first_mut()
        else {
            panic!("Call struct should have default lifetime as first generics parameter");
        };
        first_lifetime_param.lifetime = constants::DERIVED_LIFETIME.clone();
        let call_struct_type = self.type_factory.create_with_generics(
            fn_info.call_struct.item_struct.ident.clone(),
            call_struct_type_generics,
        );
        let call_stmt = Stmt::Local(
            self.local_factory.create_with_type(
                Self::CALL_VARIABLE_IDENT.clone(),
                call_struct_type,
                LocalInit {
                    eq_token: Default::default(),
                    expr: Box::new(Expr::Struct(ExprStruct {
                        attrs: Vec::new(),
                        qself: None,
                        path: self
                            .path_factory
                            .create(fn_info.call_struct.item_struct.ident.clone()),
                        brace_token: Default::default(),
                        fields: field_values.into_iter().collect(),
                        dot2_token: None,
                        rest: None,
                    })),
                    diverge: None,
                },
            ),
        );
        return call_stmt;
    }

    fn generate_last_stmts(
        &self,
        fn_info: &FnInfo,
        target: Target,
        maybe_containing_trait_ident: Option<&Ident>,
    ) -> Vec<Stmt> {
        let base_receiver = self.path_factory.create_expr(match target {
            Target::Other => constants::SELF_IDENT.clone(),
            Target::StaticFn(_) => Self::MOCK_VARIABLE_IDENT.clone(),
        });
        let mut handle_expr =
            self.generate_handle_expr(fn_info, base_receiver, target, maybe_containing_trait_ident);
        if fn_info.parent.has_return_value() {
            handle_expr = Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(handle_expr)),
            });
        }
        let handle_stmt = Stmt::Expr(handle_expr, Some(Default::default()));
        let last_stmts = match target {
            Target::Other => vec![handle_stmt],
            Target::StaticFn(mock_type) => {
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

    fn generate_handle_expr(
        &self,
        fn_info: &FnInfo,
        base_receiver: Expr,
        target: Target,
        maybe_containing_trait_ident: Option<&Ident>,
    ) -> Expr {
        let idents = vec![
            constants::DATA_IDENT.clone(),
            fn_info.data_field_ident.clone(),
        ];
        let method_name = match (
            fn_info.parent.maybe_base_fn_block.is_some(),
            fn_info.parent.has_return_value(),
        ) {
            (false, false) => Self::HANDLE_METHOD_IDENT.clone(),
            (false, true) => Self::HANDLE_RETURNING_METHOD_IDENT.clone(),
            (true, false) => Self::HANDLE_BASE_METHOD_IDENT.clone(),
            (true, true) => Self::HANDLE_BASE_RETURNING_METHOD_IDENT.clone(),
        };
        let args = if fn_info.parent.maybe_base_fn_block.is_some() {
            let specific_fn_info_ident = match maybe_containing_trait_ident {
                None => &fn_info.parent.fn_ident,
                Some(containing_trait_ident) => {
                    &format_ident!("{}_{}", containing_trait_ident, fn_info.parent.fn_ident)
                }
            };
            let base_fn_ident = self.base_fn_ident_formatter.format(specific_fn_info_ident);
            let base_fn_path = match target {
                Target::StaticFn(_) => self.path_factory.create_expr(base_fn_ident),
                Target::Other => self.path_factory.create_expr_from_parts(vec![
                    constants::SELF_TYPE_IDENT.clone(),
                    base_fn_ident,
                ]),
            };
            vec![
                self.expr_reference_factory.create(base_receiver.clone()),
                self.path_factory
                    .create_expr(Self::CALL_VARIABLE_IDENT.clone()),
                base_fn_path,
            ]
        } else {
            vec![
                self.expr_reference_factory.create(base_receiver.clone()),
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
enum Target<'a> {
    Other,
    StaticFn(&'a MockType),
}
