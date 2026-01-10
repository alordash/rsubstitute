use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::input_args_generator::IInputArgsGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use std::sync::Arc;
use syn::*;

pub trait IMockReceivedImplGenerator {
    fn generate(
        &self,
        mock_received_struct: &MockReceivedStruct,
        fn_infos: &[FnInfo],
    ) -> MockReceivedImpl;
}

pub(crate) struct MockReceivedImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub impl_factory: Arc<dyn IImplFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub input_args_generator: Arc<dyn IInputArgsGenerator>,
}

impl IMockReceivedImplGenerator for MockReceivedImplGenerator {
    fn generate(
        &self,
        mock_received_struct: &MockReceivedStruct,
        fn_infos: &[FnInfo],
    ) -> MockReceivedImpl {
        let self_ty = self
            .type_factory
            .create(mock_received_struct.item_struct.ident.clone());
        let fn_receiveds = fn_infos
            .iter()
            .map(|x| ImplItem::Fn(self.generate_fn_received(x)))
            .collect();

        let item_impl = self
            .impl_factory
            .create_with_default_lifetime(self_ty, fn_receiveds);
        let mock_received_impl = MockReceivedImpl { item_impl };
        return mock_received_impl;
    }
}

impl MockReceivedImplGenerator {
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
            ident: fn_info.parent.ident.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: iter::once(constants::REF_SELF_ARG_WITH_LIFETIME.clone())
                .chain(self.input_args_generator.generate_input_args(fn_info))
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
        let (args_checker_var_ident, args_checker_decl_stmt) = self
            .input_args_generator
            .generate_args_checker_var_ident_and_decl_stmt(fn_info);
        let verify_received_stmt = Stmt::Expr(
            Expr::MethodCall(self.expr_method_call_factory.create(
                vec![
                    constants::SELF_IDENT.clone(),
                    constants::DATA_IDENT.clone(),
                    fn_info.data_field_ident.clone(),
                ],
                constants::FN_DATA_VERIFY_RECEIVED_FN_IDENT.clone(),
                vec![args_checker_var_ident, Self::TIMES_ARG_IDENT.clone()],
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
}
