use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait IMockConstructorBlockGenerator {
    fn generate_for_trait(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> Block;

    fn generate_for_static(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        base_caller_struct: &BaseCallerStruct,
    ) -> Block;
}

pub(crate) struct MockConstructorBlockGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
}

impl IMockConstructorBlockGenerator for MockConstructorBlockGenerator {
    fn generate_for_trait(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> Block {
        let result = self.generate(
            mock_struct,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            None,
        );
        return result;
    }

    fn generate_for_static(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        base_caller_struct: &BaseCallerStruct,
    ) -> Block {
        let result = self.generate(
            mock_struct,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            Some(base_caller_struct),
        );
        return result;
    }
}

impl MockConstructorBlockGenerator {
    const DATA_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("data"));

    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        maybe_base_caller_struct: Option<&BaseCallerStruct>,
    ) -> Block {
        let phantom_lifetime_field = constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD_VALUE.clone();
        let mut data_fields: Vec<_> = mock_data_struct
            .field_and_fn_idents
            .iter()
            .map(|(field_ident, fn_ident)| {
                FieldValue {
                    attrs: Vec::new(),
                    // TODO - do something with this "expect", it appears more than one time
                    member: Member::Named(field_ident.clone()),
                    colon_token: Some(Default::default()),
                    expr: Expr::Call(ExprCall {
                        attrs: Vec::new(),
                        func: Box::new(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: constants::FN_DATA_TYPE_IDENT.clone(),
                                        arguments: PathArguments::None,
                                    },
                                    PathSegment {
                                        ident: constants::NEW_IDENT.clone(),
                                        arguments: PathArguments::None,
                                    },
                                ]
                                .into_iter()
                                .collect(),
                            },
                        })),
                        paren_token: Default::default(),
                        args: [
                            Expr::Lit(ExprLit {
                                attrs: Vec::new(),
                                lit: Lit::Str(LitStr::new(
                                    &fn_ident.to_string(),
                                    Span::call_site(),
                                )),
                            }),
                            constants::SERVICES_REF_EXPR.clone(),
                        ]
                        .into_iter()
                        .collect(),
                    }),
                }
            })
            .collect();
        data_fields.insert(0, phantom_lifetime_field);
        if let Some(base_caller_struct) = maybe_base_caller_struct {
            let base_caller_field = FieldValue {
                attrs: Vec::new(),
                member: Member::Named(constants::BASE_CALLER_FIELD_IDENT.clone()),
                colon_token: Some(Default::default()),
                expr: Expr::Call(ExprCall {
                    attrs: Vec::new(),
                    func: Box::new(Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: self.path_factory.create_from_parts(&[
                            constants::ARC_IDENT.clone(),
                            constants::NEW_IDENT.clone(),
                        ]),
                    })),
                    paren_token: Default::default(),
                    args: [Expr::Call(ExprCall {
                        attrs: Vec::new(),
                        func: Box::new(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: self.path_factory.create_from_parts(&[
                                constants::REF_CELL_IDENT.clone(),
                                constants::NEW_IDENT.clone(),
                            ]),
                        })),
                        paren_token: Default::default(),
                        args: [Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: self
                                .path_factory
                                .create(base_caller_struct.item_struct.ident.clone()),
                        })]
                        .into_iter()
                        .collect(),
                    })]
                    .into_iter()
                    .collect(),
                }),
            };
            data_fields.insert(1, base_caller_field);
        }
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
                    func: Box::new(Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: self.path_factory.create_from_parts(&[
                            constants::ARC_IDENT.clone(),
                            constants::NEW_IDENT.clone(),
                        ]),
                    })),
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
        let return_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: returned_type_path,
                    brace_token: Default::default(),
                    fields: [
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
                                fields: [constants::DATA_FIELD_VALUE.clone()].into_iter().collect(),
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
                                fields: [constants::DATA_FIELD_VALUE.clone()].into_iter().collect(),
                                dot2_token: None,
                                rest: None,
                            }),
                        },
                        constants::DATA_SHORT_FIELD_VALUE.clone(),
                    ]
                    .into_iter()
                    .collect(),
                    dot2_token: None,
                    rest: None,
                }))),
            }),
            Some(Default::default()),
        );
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![data_stmt, return_stmt],
        };
        return block;
    }
}
