use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::mock_generation::parameters::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use syn::*;

pub(crate) fn generate_for_trait(
    mock_type: &MockType,
    mock_setup_struct: &MockSetupStruct,
    fn_infos: &[FnInfo],
) -> MockSetupImpl {
    let self_ty = mock_setup_struct.ty.clone();
    let fn_setups = fn_infos
        .iter()
        .map(|x| {
            let output_type = setup_output::generate_for_trait(mock_type, x);
            return ImplItem::Fn(generate_fn_setup(x, mock_type, output_type, Target::Trait));
        })
        .collect();

    let item_impl = r#impl::create_with_default_lifetime(mock_type, self_ty, fn_setups);
    let mock_setup_impl = MockSetupImpl { item_impl };
    return mock_setup_impl;
}

pub(crate) fn generate_for_static(
    mock_type: &MockType,
    mock_setup_struct: &MockSetupStruct,
    fn_info: &FnInfo,
) -> MockSetupImpl {
    let self_ty = mock_setup_struct.ty.clone();
    let output_type = setup_output::generate_for_trait(mock_type, fn_info);
    let fn_setup = ImplItem::Fn(generate_fn_setup(
        fn_info,
        mock_type,
        output_type,
        Target::Static,
    ));

    let item_impl = r#impl::create_with_default_lifetime(mock_type, self_ty, vec![fn_setup]);
    let mock_setup_impl = MockSetupImpl { item_impl };
    return mock_setup_impl;
}

const FN_TUNER_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("fn_tuner"));

fn generate_fn_setup(
    fn_info: &FnInfo,
    mock_type: &MockType,
    output_type: TypePath,
    target: Target,
) -> ImplItemFn {
    let block = generate_fn_setup_block(fn_info, &output_type);
    let mut generics = match target {
        Target::Trait => fn_info.parent.own_generics.clone(),
        Target::Static => Default::default(),
    };
    generics = generics.with_head_lifetime_param(constants::PLACEHOLDER_LIFETIME_PARAM.clone());
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: match target {
            Target::Trait => fn_info.parent.fn_ident.clone(),
            Target::Static => constants::MOCK_SETUP_FIELD_IDENT.clone(),
        },
        generics,
        paren_token: Default::default(),
        inputs: iter::once(constants::REF_SELF_ARG.clone())
            .chain(input_args::generate_input_args(
                fn_info,
                fn_info
                    .parent
                    .get_internal_phantom_types_count()
                    + mock_type.generics.get_phantom_fields_count(),
            ))
            .collect(),
        variadic: None,
        output: ReturnType::Type(Default::default(), Box::new(Type::Path(output_type))),
    };
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Default::default()),
        defaultness: None,
        sig,
        block,
    };
    return impl_item_fn;
}

fn generate_fn_setup_block(fn_info: &FnInfo, output_type: &TypePath) -> Block {
    let (args_checker_var_ident, args_checker_decl_stmt) =
        input_args::generate_args_checker_var_ident_and_decl_stmt(fn_info);
    let mut fn_tuner_type = output_type.clone();
    let PathArguments::AngleBracketed(ref mut fn_tuner_type_generics) =
        fn_tuner_type.path.segments[0].arguments
    else {
        panic!("Setup function return type (FnTuner) must have generics.")
    };
    let GenericArgument::Lifetime(ref mut fn_tuner_lifetime) = fn_tuner_type_generics.args[0]
    else {
        panic!("Setup function return type (FnTuner) must have lifetime as first generic parameter")
    };
    *fn_tuner_lifetime = Lifetime::new("'_", Span::call_site());
    let fn_tuner_decl_stmt = Stmt::Local(local::create_with_type(
        FN_TUNER_VAR_IDENT.clone(),
        Type::Path(fn_tuner_type),
        LocalInit {
            eq_token: Default::default(),
            expr: Box::new(Expr::MethodCall(expr_method_call::create(
                vec![
                    constants::SELF_IDENT.clone(),
                    constants::DATA_IDENT.clone(),
                    fn_info.data_field_ident.clone(),
                ],
                constants::FN_DATA_ADD_CONFIG_FN_IDENT.clone(),
                vec![args_checker_var_ident, constants::SELF_IDENT.clone()],
            ))),
            diverge: None,
        },
    ));
    let return_stmt = Stmt::Expr(
        Expr::Return(ExprReturn {
            attrs: Vec::new(),
            return_token: Default::default(),
            expr: Some(Box::new(transmute_lifetime_expr::create_for_expr(
                path::create_expr(FN_TUNER_VAR_IDENT.clone()),
            ))),
        }),
        Some(Default::default()),
    );
    let stmts = vec![args_checker_decl_stmt, fn_tuner_decl_stmt, return_stmt];
    let block = Block {
        brace_token: Default::default(),
        stmts,
    };
    return block;
}
