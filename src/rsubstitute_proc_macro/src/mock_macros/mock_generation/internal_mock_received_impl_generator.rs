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

pub trait IInternalMockReceivedImplGenerator {
    fn generate(
        &self,
        mock_received_struct: &MockReceivedStruct,
        fn_infos: &[FnInfo],
    ) -> InternalMockReceivedImpl;
}

pub(crate) struct InternalMockReceivedImplGenerator {
    pub path_factory: Rc<dyn IPathFactory>,
    pub type_factory: Rc<dyn ITypeFactory>,
    pub field_value_factory: Rc<dyn IFieldValueFactory>,
    pub local_factory: Rc<dyn ILocalFactory>,
    pub expr_method_call_factory: Rc<dyn IExprMethodCallFactory>,
    pub reference_normalizer: Rc<dyn IReferenceNormalizer>,
}

impl IInternalMockReceivedImplGenerator for InternalMockReceivedImplGenerator {
    fn generate(
        &self,
        mock_received_struct: &MockReceivedStruct,
        fn_infos: &[FnInfo],
    ) -> InternalMockReceivedImpl {
        let self_ty = self
            .type_factory
            .create(mock_received_struct.item_struct.ident.clone());
        let fn_receiveds = fn_infos
            .iter()
            .map(|x| ImplItem::Fn(self.generate_fn_received(x)))
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
            items: fn_receiveds,
        };
        self.reference_normalizer.normalize_in_impl(
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            &mut item_impl,
        );
        let internal_mock_received_impl = InternalMockReceivedImpl { item_impl };
        return internal_mock_received_impl;
    }
}

impl InternalMockReceivedImplGenerator {
    const ARGS_CHECKER_VARIABLE_SUFFIX: &'static str = "args_checker";
    const RECEIVED_FN_PREFIX: &'static str = "received";
    const TIMES_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("times"));
    const TIMES_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Times"));

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
            ident: format_ident!("{}_{}", Self::RECEIVED_FN_PREFIX, fn_info.parent.ident),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: iter::once(constants::REF_SELF_ARG_WITH_LIFETIME.clone())
                .chain(self.generate_input_args(fn_info))
                .chain(iter::once(times_arg))
                .collect(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(Type::Reference(TypeReference {
                    and_token: Default::default(),
                    lifetime: Some(constants::DEFAULT_ARG_FIELD_LIFETIME.clone()),
                    mutability: None,
                    elem: Box::new(constants::SELF_TYPE.clone()),
                })),
            ),
        };
        let block = self.generate_fn_received_block(fn_info);
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

    fn generate_fn_received_block(&self, fn_info: &FnInfo) -> Block {
        let (args_checker_var_ident, args_checker_decl_stmt) =
            self.generate_args_checker_var_ident_and_decl_stmt(fn_info);
        let verify_received_stmt = Stmt::Expr(
            Expr::MethodCall(self.expr_method_call_factory.create(
                &[
                    constants::SELF_IDENT.clone(),
                    constants::DATA_IDENT.clone(),
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
            .args_checker_struct
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
