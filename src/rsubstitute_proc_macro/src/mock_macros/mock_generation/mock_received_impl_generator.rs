use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::input_args_generator::IInputArgsGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub trait IMockReceivedImplGenerator {
    fn generate_for_trait(
        &self,
        mock_type: &MockType,
        mock_received_struct: &MockReceivedStruct,
        fn_infos: &[FnInfo],
    ) -> MockReceivedImpl;

    fn generate_for_struct_trait(
        &self,
        mock_type: &MockType,
        mock_received_struct: &MockReceivedStruct,
        fn_infos: &[FnInfo],
    ) -> MockReceivedImpl;

    fn generate_for_static(
        &self,
        mock_type: &MockType,
        mock_received_struct: &MockReceivedStruct,
        fn_info: &FnInfo,
    ) -> MockReceivedImpl;
}

pub(crate) struct MockReceivedImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub impl_factory: Arc<dyn IImplFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub input_args_generator: Arc<dyn IInputArgsGenerator>,
    pub received_signature_generator: Arc<dyn IReceivedSignatureGenerator>,
}

impl IMockReceivedImplGenerator for MockReceivedImplGenerator {
    fn generate_for_trait(
        &self,
        mock_type: &MockType,
        mock_received_struct: &MockReceivedStruct,
        fn_infos: &[FnInfo],
    ) -> MockReceivedImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_received_struct.item_struct);
        let fns = fn_infos
            .iter()
            .map(|x| ImplItem::Fn(self.generate_fn_received(x, mock_type)))
            .chain(std::iter::once(self.generate_only_fn()))
            .collect();

        let item_impl = self
            .impl_factory
            .create_with_default_lifetime(mock_type, self_ty, fns);
        let mock_received_impl = MockReceivedImpl { item_impl };
        return mock_received_impl;
    }

    fn generate_for_struct_trait(
        &self,
        mock_type: &MockType,
        mock_received_struct: &MockReceivedStruct,
        fn_infos: &[FnInfo],
    ) -> MockReceivedImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_received_struct.item_struct);
        let fns = fn_infos
            .iter()
            .map(|x| ImplItem::Fn(self.generate_fn_received(x, mock_type)))
            .collect();

        let item_impl = self
            .impl_factory
            .create_with_default_lifetime(mock_type, self_ty, fns);
        let mock_received_impl = MockReceivedImpl { item_impl };
        return mock_received_impl;
    }

    fn generate_for_static(
        &self,
        mock_type: &MockType,
        mock_received_struct: &MockReceivedStruct,
        fn_info: &FnInfo,
    ) -> MockReceivedImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_received_struct.item_struct);
        let mut fn_received = self.generate_fn_received(fn_info, mock_type);
        fn_received.sig.ident = constants::MOCK_RECEIVED_FIELD_IDENT.clone();
        let only_fn = self.generate_only_fn();

        let item_impl = self.impl_factory.create_with_default_lifetime(
            mock_type,
            self_ty,
            vec![ImplItem::Fn(fn_received), only_fn],
        );
        let mock_received_impl = MockReceivedImpl { item_impl };
        return mock_received_impl;
    }
}

impl MockReceivedImplGenerator {
    fn generate_fn_received(&self, fn_info: &FnInfo, mock_type: &MockType) -> ImplItemFn {
        let sig = self
            .received_signature_generator
            .generate_for_trait(fn_info, mock_type);
        let block = self.generate_fn_received_block(fn_info);
        let impl_item_fn = ImplItemFn {
            attrs: vec![
                constants::ALLOW_UNUSED_ATTRIBUTE.clone(),
                constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone(),
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
                vec![
                    args_checker_var_ident,
                    self.received_signature_generator.get_times_arg_ident(),
                ],
            )),
            Some(Default::default()),
        );
        let stmts = vec![
            args_checker_decl_stmt,
            verify_received_stmt,
            constants::RETURN_SELF_STMT.clone(),
        ];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }

    fn generate_only_fn(&self) -> ImplItem {
        let verify_received_nothing_else_stmt = Stmt::Expr(
            Expr::MethodCall(self.expr_method_call_factory.create(
                vec![constants::SELF_IDENT.clone(), constants::DATA_IDENT.clone()],
                format_ident!("verify_received_nothing_else"),
                Vec::new(),
            )),
            Some(Default::default()),
        );
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: format_ident!("no_other_calls"),
                generics: Generics::default(),
                paren_token: Default::default(),
                inputs: [constants::REF_SELF_ARG_WITH_LIFETIME.clone()]
                    .into_iter()
                    .collect(),
                variadic: None,
                output: ReturnType::Default,
            },
            block: Block {
                brace_token: Default::default(),
                stmts: vec![verify_received_nothing_else_stmt],
            },
        };
        return ImplItem::Fn(impl_item_fn);
    }
}
