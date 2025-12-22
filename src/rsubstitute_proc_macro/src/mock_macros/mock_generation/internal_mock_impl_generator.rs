use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::{InternalMockImplInfo, MockStructInfo};
use crate::syntax::{
    IExprMethodCallFactory, IFieldValueFactory, ILocalFactory, IPathFactory, ITypeFactory,
};
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use std::rc::Rc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IInternalMockImplGenerator {
    fn generate(
        &self,
        mock_struct_info: &MockStructInfo,
        fn_infos: &[FnInfo],
    ) -> InternalMockImplInfo;
}

pub struct InternalMockImplGenerator {
    pub(crate) path_factory: Rc<dyn IPathFactory>,
    pub(crate) type_factory: Rc<dyn ITypeFactory>,
    pub(crate) field_value_factory: Rc<dyn IFieldValueFactory>,
    pub(crate) local_factory: Rc<dyn ILocalFactory>,
    pub(crate) expr_method_call_factory: Rc<dyn IExprMethodCallFactory>,
}

impl IInternalMockImplGenerator for InternalMockImplGenerator {
    fn generate(
        &self,
        mock_struct_info: &MockStructInfo,
        fn_infos: &[FnInfo],
    ) -> InternalMockImplInfo {
        let self_ty = self
            .type_factory
            .create(mock_struct_info.item_struct.ident.clone());
        let constructor = self.generate_constructor(mock_struct_info);
        let fn_setups = fn_infos
            .iter()
            .map(|x| ImplItem::Fn(self.generate_fn_setup(x)));
        let fn_receiveds = fn_infos
            .iter()
            .map(|x| ImplItem::Fn(self.generate_fn_received(x)));
        let items = std::iter::once(constructor)
            .chain(fn_setups)
            .chain(fn_receiveds)
            .collect();
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: Default::default(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };

        let internal_mock_impl_info = InternalMockImplInfo { item_impl };
        return internal_mock_impl_info;
    }
}

impl InternalMockImplGenerator {
    const CONSTRUCTOR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));
    const ARGS_CHECKER_VARIABLE_SUFFIX: &'static str = "args_checker";
    const FN_CONFIG_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("fn_config"));
    const SHARED_FN_CONFIG_VAR_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("shared_fn_config"));
    const RECEIVED_FN_PREFIX: &'static str = "received";
    const TIMES_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("times"));
    const TIMES_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Times"));

    fn generate_constructor(&self, mock_struct_info: &MockStructInfo) -> ImplItem {
        let block = self.generate_constructor_block(mock_struct_info);
        let item_impl = ImplItem::Fn(ImplItemFn {
            attrs: Vec::new(),
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

    fn generate_constructor_block(&self, mock_struct_info: &MockStructInfo) -> Block {
        let fields = mock_struct_info.item_struct.fields.iter().map(|field| {
            let field_ident = field
                .ident
                .clone()
                .expect("Field in call struct should be named");
            FieldValue {
                attrs: Vec::new(),
                // TODO - do something with this "expect", it appears more than one time
                member: Member::Named(field_ident.clone()),
                colon_token: Some(Default::default()),
                // expr: constants::DEFAULT_INVOKE_EXPR.clone(),
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
                                    ident: constants::FN_DATA_NEW_FN_IDENT.clone(),
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
                            lit: Lit::Str(LitStr::new(&field_ident.to_string(), Span::call_site())),
                        }),
                        constants::SERVICES_REF_EXPR.clone(),
                    ]
                    .into_iter()
                    .collect(),
                }),
            }
        });
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![Stmt::Expr(
                Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: constants::SELF_TYPE_PATH.clone(),
                    brace_token: Default::default(),
                    fields: fields.collect(),
                    dot2_token: None,
                    rest: None,
                }),
                None,
            )],
        };
        return block;
    }

    fn generate_fn_setup(&self, fn_info: &FnInfo) -> ImplItemFn {
        let sig =
            Signature {
                // TODO - all these `None` should be actually mapped to source fns signature
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: fn_info.parent.ident.clone(),
                generics: Generics::default(),
                paren_token: Default::default(),
                inputs: iter::once(constants::REF_SELF_ARG.clone())
                    .chain(self.generate_input_args(fn_info))
                    .collect(),
                variadic: None,
                output: ReturnType::Type(
                    Default::default(),
                    Box::new(Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [PathSegment {
                                ident: constants::SHARED_FN_CONFIG_TYPE_IDENT.clone(),
                                arguments: PathArguments::AngleBracketed(
                                    AngleBracketedGenericArguments {
                                        colon2_token: None,
                                        lt_token: Default::default(),
                                        args: [
                                            GenericArgument::Lifetime(Lifetime {
                                                apostrophe: Span::call_site(),
                                                ident: constants::DISCARD_IDENT.clone(),
                                            }),
                                            GenericArgument::Type(self.type_factory.create(
                                                fn_info.call_info.item_struct.ident.clone(),
                                            )),
                                            GenericArgument::Type(self.type_factory.create(
                                                fn_info.args_checker_info.item_struct.ident.clone(),
                                            )),
                                            GenericArgument::Type(
                                                fn_info.parent.get_return_value_type(),
                                            ),
                                            GenericArgument::Type(constants::SELF_TYPE.clone()),
                                        ]
                                        .into_iter()
                                        .collect(),
                                        gt_token: Default::default(),
                                    },
                                ),
                            }]
                            .into_iter()
                            .collect(),
                        },
                    })),
                ),
            };
        let block = self.generate_fn_setup_block(fn_info);
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig,
            block,
        };
        return impl_item_fn;
    }

    fn generate_fn_setup_block(&self, fn_info: &FnInfo) -> Block {
        let (args_checker_var_ident, args_checker_decl_stmt) =
            self.generate_args_checker_var_ident_and_decl_stmt(fn_info);
        let fn_config_decl_stmt = Stmt::Local(self.local_factory.create(
            Self::FN_CONFIG_VAR_IDENT.clone(),
            LocalInit {
                eq_token: Default::default(),
                expr: Box::new(Expr::MethodCall(self.expr_method_call_factory.create(
                    &[
                        constants::SELF_IDENT.clone(),
                        fn_info.data_field_ident.clone(),
                    ],
                    constants::FN_DATA_ADD_CONFIG_FN_IDENT.clone(),
                    &[args_checker_var_ident],
                ))),
                diverge: None,
            },
        ));
        let shared_fn_config_decl_stmt = Stmt::Local(
            self.local_factory.create(
                Self::SHARED_FN_CONFIG_VAR_IDENT.clone(),
                LocalInit {
                    eq_token: Default::default(),
                    expr: Box::new(Expr::Call(ExprCall {
                        attrs: Vec::new(),
                        func: Box::new(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: constants::SHARED_FN_CONFIG_TYPE_IDENT.clone(),
                                        arguments: PathArguments::None,
                                    },
                                    PathSegment {
                                        ident: constants::SHARED_FN_CONFIG_NEW_FN_IDENT.clone(),
                                        arguments: PathArguments::None,
                                    },
                                ]
                                .into_iter()
                                .collect(),
                            },
                        })),
                        paren_token: Default::default(),
                        // TODO - add factory for ExprPath
                        args: [
                            Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: self.path_factory.create(Self::FN_CONFIG_VAR_IDENT.clone()),
                            }),
                            Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: constants::SELF_IDENT_PATH.clone(),
                            }),
                        ]
                        .into_iter()
                        .collect(),
                    })),
                    diverge: None,
                },
            ),
        );
        let return_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: self
                        .path_factory
                        .create(Self::SHARED_FN_CONFIG_VAR_IDENT.clone()),
                }))),
            }),
            Some(Default::default()),
        );
        let stmts = vec![
            args_checker_decl_stmt,
            fn_config_decl_stmt,
            shared_fn_config_decl_stmt,
            return_stmt,
        ];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }

    fn generate_fn_received(&self, fn_info: &FnInfo) -> ImplItemFn {
        let times_arg = FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: Self::TIMES_ARG_IDENT.clone(),
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: Box::new(self.type_factory.create(Self::TIMES_TYPE_IDENT.clone())),
        });
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: format_ident!("{}_{}", fn_info.parent.ident, Self::RECEIVED_FN_PREFIX),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: iter::once(constants::REF_SELF_ARG.clone())
                .chain(self.generate_input_args(fn_info))
                .chain(iter::once(times_arg))
                .collect(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(Type::Reference(TypeReference {
                    and_token: Default::default(),
                    lifetime: None,
                    mutability: None,
                    elem: Box::new(constants::SELF_TYPE.clone()),
                })),
            ),
        };
        let block = self.generate_fn_received_block(fn_info);
        let impl_item_fn = ImplItemFn {
            attrs: Default::default(),
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig,
            block,
        };
        return impl_item_fn;
    }

    fn generate_fn_received_block(&self, fn_info: &FnInfo) -> Block {
        let (args_checker_var_ident, args_checker_decl_stmt) =
            self.generate_args_checker_var_ident_and_decl_stmt(fn_info);
        let verify_received_stmt = Stmt::Expr(
            Expr::MethodCall(self.expr_method_call_factory.create(
                &[
                    constants::SELF_IDENT.clone(),
                    fn_info.data_field_ident.clone(),
                ],
                constants::FN_DATA_VERIFY_RECEIVED_FN_IDENT.clone(),
                &[args_checker_var_ident, Self::TIMES_ARG_IDENT.clone()],
            )),
            Some(Default::default()),
        );
        let return_self_stmt = Stmt::Expr(
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: constants::SELF_IDENT_PATH.clone(),
                }))),
            }),
            Some(Default::default()),
        );
        let stmts = vec![
            args_checker_decl_stmt,
            verify_received_stmt,
            return_self_stmt,
        ];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }

    fn generate_input_args(&self, fn_info: &FnInfo) -> impl Iterator<Item = FnArg> {
        return fn_info
            .args_checker_info
            .item_struct
            .fields
            .iter()
            .map(|field| {
                FnArg::Typed(PatType {
                    attrs: Vec::new(),
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: field
                            .ident
                            .clone()
                            .expect("Field in args checker struct should be named"),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(field.ty.clone()),
                })
            });
    }

    fn generate_args_checker_var_ident_and_decl_stmt(&self, fn_info: &FnInfo) -> (Ident, Stmt) {
        let args_checker_var_ident = format_ident!(
            "{}_{}",
            fn_info.parent.ident,
            Self::ARGS_CHECKER_VARIABLE_SUFFIX
        );
        let args_checker_decl_stmt = Stmt::Local(
            self.local_factory.create(
                args_checker_var_ident.clone(),
                LocalInit {
                    eq_token: Default::default(),
                    expr: Box::new(Expr::Struct(ExprStruct {
                        attrs: Vec::new(),
                        qself: None,
                        path: self
                            .path_factory
                            .create(fn_info.args_checker_info.item_struct.ident.clone()),
                        brace_token: Default::default(),
                        fields: fn_info
                            .args_checker_info
                            .item_struct
                            .fields
                            .iter()
                            .map(|field| self.field_value_factory.create(field))
                            .collect(),
                        dot2_token: None,
                        rest: None,
                    })),
                    diverge: None,
                },
            ),
        );
        return (args_checker_var_ident, args_checker_decl_stmt);
    }
}
