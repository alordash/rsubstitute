use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use std::rc::Rc;
use syn::*;

pub trait IInternalMockSetupImplGenerator {
    fn generate(
        &self,
        mock_setup_struct: &MockSetupStruct,
        fn_infos: &[FnInfo],
    ) -> InternalMockSetupImpl;
}

pub(crate) struct InternalMockSetupImplGenerator {
    pub path_factory: Rc<dyn IPathFactory>,
    pub type_factory: Rc<dyn ITypeFactory>,
    pub field_value_factory: Rc<dyn IFieldValueFactory>,
    pub local_factory: Rc<dyn ILocalFactory>,
    pub expr_method_call_factory: Rc<dyn IExprMethodCallFactory>,
    pub reference_normalizer: Rc<dyn IReferenceNormalizer>,
}

impl IInternalMockSetupImplGenerator for InternalMockSetupImplGenerator {
    fn generate(
        &self,
        mock_setup_struct: &MockSetupStruct,
        fn_infos: &[FnInfo],
    ) -> InternalMockSetupImpl {
        let self_ty = self
            .type_factory
            .create(mock_setup_struct.item_struct.ident.clone());
        let fn_setups = fn_infos
            .iter()
            .map(|x| ImplItem::Fn(self.generate_fn_setup(x)))
            .collect();

        let mut item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items: fn_setups,
        };
        self.reference_normalizer.normalize_in_impl(
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            &mut item_impl,
        );
        let internal_mock_setup_impl = InternalMockSetupImpl { item_impl };
        return internal_mock_setup_impl;
    }
}

impl InternalMockSetupImplGenerator {
    const ARGS_CHECKER_VARIABLE_SUFFIX: &'static str = "args_checker";
    const FN_CONFIG_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("fn_config"));
    const SHARED_FN_CONFIG_VAR_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("shared_fn_config"));

    fn generate_fn_setup(&self, fn_info: &FnInfo) -> ImplItemFn {
        let sig = Signature {
            // TODO - all these `None` should be actually mapped to source fns signature
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_info.parent.ident.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: iter::once(constants::REF_SELF_ARG_WITH_LIFETIME.clone())
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
                                        GenericArgument::Lifetime(
                                            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
                                        ),
                                        GenericArgument::Type(
                                            self.type_factory.create(
                                                fn_info.call_struct.item_struct.ident.clone(),
                                            ),
                                        ),
                                        GenericArgument::Type(self.type_factory.create(
                                            fn_info.args_checker_struct.item_struct.ident.clone(),
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
            attrs: vec![
                constants::ALLOW_UNUSED_ATTRIBUTE.clone(),
                constants::ALLOW_ELIDED_NAMED_LIFETIMES_ATTRIBUTE.clone(),
            ],
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
                        constants::DATA_IDENT.clone(),
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

    // TODO - unite this and next fn with same in received_impl_generator
    fn generate_input_args(&self, fn_info: &FnInfo) -> impl Iterator<Item = FnArg> {
        return fn_info
            .args_checker_struct
            .item_struct
            .fields
            .iter()
            .skip(1)
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
        let fn_fields: Vec<_> = fn_info
            .args_checker_struct
            .item_struct
            .fields
            .iter()
            .skip(1)
            .map(|field| self.field_value_factory.create(field))
            .collect();
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
                            .create(fn_info.args_checker_struct.item_struct.ident.clone()),
                        brace_token: Default::default(),
                        fields: std::iter::once(
                            constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD_VALUE.clone(),
                        )
                        .chain(fn_fields)
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
