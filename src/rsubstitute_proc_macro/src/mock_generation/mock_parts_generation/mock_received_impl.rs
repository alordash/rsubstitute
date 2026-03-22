use crate::constants;
use crate::mock_generation::fn_info_generation::models::FnInfo;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_for_trait(
    mock_type: &MockType,
    mock_received_struct: &MockReceivedStruct,
    fn_infos: &[FnInfo],
) -> MockReceivedImpl {
    let self_ty = mock_received_struct.ty.clone();
    let fns = fn_infos
        .iter()
        .map(|x| {
            ImplItem::Fn(generate_fn_received(
                mock_type,
                x,
                OutputTypeGenerics::UseFnOwn,
            ))
        })
        .chain(std::iter::once(generate_only_fn()))
        .collect();

    let item_impl = r#impl::create_with_default_lifetime(mock_type, self_ty, fns);
    let mock_received_impl = MockReceivedImpl { item_impl };
    return mock_received_impl;
}

pub(crate) fn generate_for_struct_trait(
    mock_type: &MockType,
    mock_received_struct: &MockReceivedStruct,
    fn_infos: &[FnInfo],
) -> MockReceivedImpl {
    let self_ty = mock_received_struct.ty.clone();
    let fns = fn_infos
        .iter()
        .map(|x| {
            ImplItem::Fn(generate_fn_received(
                mock_type,
                x,
                OutputTypeGenerics::UseFnOwn,
            ))
        })
        .collect();

    let item_impl = r#impl::create_with_default_lifetime(mock_type, self_ty, fns);
    let mock_received_impl = MockReceivedImpl { item_impl };
    return mock_received_impl;
}

pub(crate) fn generate_for_static(
    mock_type: &MockType,
    mock_received_struct: &MockReceivedStruct,
    fn_info: &FnInfo,
) -> MockReceivedImpl {
    let self_ty = mock_received_struct.ty.clone();
    let mut fn_received = generate_fn_received(mock_type, fn_info, OutputTypeGenerics::DoNotUse);
    fn_received.sig.ident = constants::MOCK_RECEIVED_FIELD_IDENT.clone();
    let only_fn = generate_only_fn();

    let item_impl = r#impl::create_with_default_lifetime(
        mock_type,
        self_ty,
        vec![ImplItem::Fn(fn_received), only_fn],
    );
    let mock_received_impl = MockReceivedImpl { item_impl };
    return mock_received_impl;
}

fn generate_fn_received(
    mock_type: &MockType,
    fn_info: &FnInfo,
    output_type_generics: OutputTypeGenerics,
) -> ImplItemFn {
    let sig = received_signature::generate_for_trait(fn_info, mock_type, output_type_generics);
    let block = generate_fn_received_block(fn_info);
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Default::default()),
        defaultness: None,
        sig,
        block,
    };
    return impl_item_fn;
}

fn generate_fn_received_block(fn_info: &FnInfo) -> Block {
    let (args_checker_var_ident, args_checker_decl_stmt) =
        input_args::generate_args_checker_var_ident_and_decl_stmt(fn_info);
    let verify_received_stmt = Stmt::Expr(
        Expr::MethodCall(method_call::create(
            vec![
                constants::SELF_IDENT.clone(),
                constants::DATA_IDENT.clone(),
                fn_info.data_field_ident.clone(),
            ],
            constants::FN_DATA_VERIFY_RECEIVED_FN_IDENT.clone(),
            vec![
                args_checker_var_ident,
                received_signature::get_times_arg_ident(),
            ],
        )),
        Some(Default::default()),
    );
    let return_stmt = Stmt::Expr(
        Expr::Return(ExprReturn {
            attrs: Vec::new(),
            return_token: Default::default(),
            expr: Some(Box::new(call::create(
                constants::FN_VERIFIER_NEW_FN_EXPR.clone(),
                Expr::MethodCall(method_call::create_with_base_receiver(
                    constants::SELF_EXPR.clone(),
                    Vec::new(),
                    constants::CLONE_FN_IDENT.clone(),
                    Vec::new(),
                )),
            ))),
        }),
        Some(Default::default()),
    );
    let stmts = vec![args_checker_decl_stmt, verify_received_stmt, return_stmt];
    let block = Block {
        brace_token: Default::default(),
        stmts,
    };
    return block;
}

fn generate_only_fn() -> ImplItem {
    let verify_received_nothing_else_stmt = Stmt::Expr(
        Expr::MethodCall(method_call::create(
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
            inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
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
