use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IInternalMockImplGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> InternalMockImpl;
}

pub(crate) struct InternalMockImplGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
}

impl IInternalMockImplGenerator for InternalMockImplGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> InternalMockImpl {
        let self_ty = self
            .type_factory
            .create(mock_struct.item_struct.ident.clone());
        let constructor = self.generate_constructor(
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
        );

        let mut item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items: [constructor].into_iter().collect(),
        };
        self.reference_normalizer.normalize_in_impl(
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            &mut item_impl,
        );
        let internal_mock_impl = InternalMockImpl { item_impl };
        return internal_mock_impl;
    }
}

impl InternalMockImplGenerator {
    const CONSTRUCTOR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));
    const DATA_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("data"));

    fn generate_constructor(
        &self,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> ImplItem {
        let block = self.generate_constructor_block(
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
        );
        let item_impl = ImplItem::Fn(ImplItemFn {
            attrs: vec![constants::ALLOW_UNUSED_ATTRIBUTE.clone()],
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: Self::CONSTRUCTOR_IDENT.clone(),
                generics: Generics::default(),
                paren_token: Default::default(),
                inputs: Punctuated::new(),
                variadic: None,
                output: ReturnType::Type(
                    Default::default(),
                    Box::new(constants::SELF_TYPE.clone()),
                ),
            },
            block,
        });
        return item_impl;
    }

    fn generate_constructor_block(
        &self,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
    ) -> Block {
        let phantom_lifetime_field = constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD_VALUE.clone();
        let data_fields = mock_data_struct
            .item_struct
            .fields
            .iter()
            .skip(1) // First is phantom data
            .map(|field| {
                let field_ident = field
                    .ident
                    .clone()
                    .expect("Field in call struct should be named");
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
                                    &field_ident.to_string(),
                                    Span::call_site(),
                                )),
                            }),
                            constants::SERVICES_REF_EXPR.clone(),
                        ]
                        .into_iter()
                        .collect(),
                    }),
                }
            });
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
                        fields: std::iter::once(phantom_lifetime_field)
                            .chain(data_fields)
                            .collect(),
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
        let return_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: constants::SELF_TYPE_PATH.clone(),
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
