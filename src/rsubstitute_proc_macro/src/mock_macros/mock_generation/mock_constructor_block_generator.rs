use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait IMockConstructorBlockGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_struct_traits: Vec<&MockStructTrait>,
        maybe_inner_data_param: Option<InnerDataParam>,
    ) -> Block;
}

pub(crate) struct MockConstructorBlockGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub local_factory: Arc<dyn ILocalFactory>,
}

impl IMockConstructorBlockGenerator for MockConstructorBlockGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_struct_traits: Vec<&MockStructTrait>,
        maybe_inner_data_param: Option<InnerDataParam>,
    ) -> Block {
        let mut data_fields: Vec<_> = mock_data_struct
            .field_and_fn_idents
            .iter()
            .map(|(field_ident, fn_ident)| FieldValue {
                attrs: Vec::new(),
                member: Member::Named(field_ident.clone()),
                colon_token: Some(Default::default()),
                expr: Expr::Call(ExprCall {
                    attrs: Vec::new(),
                    func: Box::new(self.path_factory.create_expr_from_parts(vec![
                        constants::FN_DATA_TYPE_IDENT.clone(),
                        constants::NEW_IDENT.clone(),
                    ])),
                    paren_token: Default::default(),
                    args: [
                        Expr::Lit(ExprLit {
                            attrs: Vec::new(),
                            lit: Lit::Str(LitStr::new(&fn_ident.to_string(), Span::call_site())),
                        }),
                        constants::SERVICES_REF_EXPR.clone(),
                    ]
                    .into_iter()
                    .collect(),
                }),
            })
            .collect();
        let phantom_lifetime_field = constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD_VALUE.clone();
        data_fields.insert(0, phantom_lifetime_field);
        let data_stmt = Stmt::Local(Local {
            attrs: Vec::new(),
            let_token: Default::default(),
            pat: Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: Self::DATA_VAR_IDENT.clone(),
                subpat: None,
            }),
            init: Some(LocalInit {
                eq_token: Default::default(),
                expr: Box::new(Expr::Call(ExprCall {
                    attrs: Vec::new(),
                    func: Box::new(self.path_factory.create_expr_from_parts(vec![
                        constants::ARC_IDENT.clone(),
                        constants::NEW_IDENT.clone(),
                    ])),
                    paren_token: Default::default(),
                    args: [Expr::Struct(ExprStruct {
                        attrs: Vec::new(),
                        qself: None,
                        path: self
                            .path_factory
                            .create(mock_data_struct.item_struct.ident.clone()),
                        brace_token: Default::default(),
                        fields: data_fields.into_iter().collect(),
                        dot2_token: None,
                        rest: None,
                    })]
                    .into_iter()
                    .collect(),
                })),
                diverge: None,
            }),
            semi_token: Default::default(),
        });
        let returned_type_path = self
            .path_factory
            .create(mock_struct.item_struct.ident.clone());
        let mut return_stmt_fields = vec![
            FieldValue {
                attrs: Vec::new(),
                member: constants::SETUP_MEMBER.clone(),
                colon_token: Some(Default::default()),
                expr: Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: self
                        .path_factory
                        .create(mock_setup_struct.item_struct.ident.clone()),
                    brace_token: Default::default(),
                    fields: [constants::DATA_FIELD_VALUE.clone()]
                        .into_iter()
                        .chain(mock_struct_traits.iter().map(|x| {
                            self.generate_mock_trait_setup_field(
                                x.info.trait_ident_from_path.clone(),
                                &x.setup_struct.item_struct,
                            )
                        }))
                        .collect(),
                    dot2_token: None,
                    rest: None,
                }),
            },
            FieldValue {
                attrs: Vec::new(),
                member: constants::RECEIVED_MEMBER.clone(),
                colon_token: Some(Default::default()),
                expr: Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: self
                        .path_factory
                        .create(mock_received_struct.item_struct.ident.clone()),
                    brace_token: Default::default(),
                    fields: [constants::DATA_FIELD_VALUE.clone()]
                        .into_iter()
                        .chain(mock_struct_traits.iter().map(|x| {
                            self.generate_mock_trait_setup_field(
                                x.info.trait_ident_from_path.clone(),
                                &x.received_struct.item_struct,
                            )
                        }))
                        .collect(),
                    dot2_token: None,
                    rest: None,
                }),
            },
            constants::DATA_SHORT_FIELD_VALUE.clone(),
        ];
        if maybe_inner_data_param.is_some() {
            let inner_data_field = FieldValue {
                attrs: Vec::new(),
                member: Member::Named(constants::INNER_DATA_FIELD_IDENT.clone()),
                colon_token: None,
                expr: constants::EMPTY_PATH_EXPR.clone(),
            };
            return_stmt_fields.push(inner_data_field);
        };
        let return_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: returned_type_path,
                    brace_token: Default::default(),
                    fields: return_stmt_fields.into_iter().collect(),
                    dot2_token: None,
                    rest: None,
                }))),
            }),
            Some(Default::default()),
        );
        let stmts = if let Some(inner_data_param) = maybe_inner_data_param {
            let inner_data_stmt = self.generate_inner_data_stmt(&inner_data_param);
            vec![data_stmt, inner_data_stmt, return_stmt]
        } else {
            vec![data_stmt, return_stmt]
        };
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}

impl MockConstructorBlockGenerator {
    const DATA_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("data"));

    fn generate_mock_trait_setup_field(
        &self,
        trait_ident: Ident,
        mock_struct_trait_struct: &ItemStruct,
    ) -> FieldValue {
        let field_value = FieldValue {
            attrs: Vec::new(),
            member: Member::Named(trait_ident),
            colon_token: Some(Default::default()),
            expr: Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: self
                    .path_factory
                    .create(mock_struct_trait_struct.ident.clone()),
                brace_token: Default::default(),
                fields: [constants::DATA_FIELD_VALUE.clone()].into_iter().collect(),
                dot2_token: None,
                rest: None,
            }),
        };
        return field_value;
    }

    fn generate_inner_data_stmt(&self, inner_data_param: &InnerDataParam) -> Stmt {
        let local_init = LocalInit {
            eq_token: Default::default(),
            expr: Box::new(Expr::Call(ExprCall {
                attrs: Vec::new(),
                func: Box::new(self.path_factory.create_expr_from_parts(vec![
                    inner_data_param.inner_data_struct.item_struct.ident.clone(),
                    constants::NEW_IDENT.clone(),
                ])),
                paren_token: Default::default(),
                args: inner_data_param
                    .constructor_arguments
                    .iter()
                    .map(|constructor_argument| {
                        self.path_factory
                            .create_expr(constructor_argument.0.clone())
                    })
                    .collect(),
            })),
            diverge: None,
        };
        let local = self
            .local_factory
            .create(constants::INNER_DATA_FIELD_IDENT.clone(), local_init);
        let stmt = Stmt::Local(local);
        return stmt;
    }
}
