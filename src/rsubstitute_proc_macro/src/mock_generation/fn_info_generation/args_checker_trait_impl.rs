use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use std::cell::LazyCell;
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::*;

pub(crate) fn generate(
    call_struct: &CallStruct,
    args_checker_struct: &ArgsCheckerStruct,
    skipped_fields_count: usize,
) -> ArgsCheckerTraitImpl {
    let trait_ident = constants::I_ARGS_CHECKER_TRAIT_IDENT.clone();
    let trait_path = Path {
        leading_colon: None,
        segments: [PathSegment {
            ident: trait_ident,
            arguments: PathArguments::None,
        }]
        .into_iter()
        .collect(),
    };
    let self_ty = Box::new(Type::Path(args_checker_struct.ty_path.clone()));
    let items = generate_check_fn(call_struct, skipped_fields_count);
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: generics::remove_default_values(args_checker_struct.item_struct.generics.clone()),
        trait_: Some((None, trait_path, Default::default())),
        self_ty,
        brace_token: Default::default(),
        items: vec![items],
    };
    let args_checker_impl = ArgsCheckerTraitImpl { item_impl };
    return args_checker_impl;
}

const CHECK_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check"));

const ARG_CHECK_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check"));
const ARG_CHECK_REF_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check_ref"));
const ARG_CHECK_MUT_REF_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check_mut"));
const ARG_CHECK_RC_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check_rc"));
const ARG_CHECK_ARC_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check_arc"));

const DYN_CALL_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("dyn_call"));
const CALL_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));

fn generate_check_fn(call_struct: &CallStruct, skipped_fields_count: usize) -> ImplItem {
    let call_var_stmt = generate_call_var_stmt(call_struct);
    let check_stmt = generate_check_stmt(call_struct, skipped_fields_count);
    let block = Block {
        brace_token: Default::default(),
        stmts: vec![call_var_stmt, check_stmt],
    };
    let impl_item = ImplItem::Fn(ImplItemFn {
        attrs: vec![constants::ALLOW_UNUSED_ATTRIBUTE.clone()],
        vis: Visibility::Inherited,
        defaultness: None,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: CHECK_FN_IDENT.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: [
                constants::REF_SELF_ARG.clone(),
                FnArg::Typed(PatType {
                    attrs: Vec::new(),
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: DYN_CALL_ARG_IDENT.clone(),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(constants::DYN_CALL_REF_TYPE.clone()),
                }),
            ]
            .into_iter()
            .collect(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(constants::VEC_OF_ARG_CHECK_RESULT_TYPE.clone()),
            ),
        },
        block,
    });
    return impl_item;
}

fn generate_call_var_stmt(call_struct: &CallStruct) -> Stmt {
    let call_var_type = r#type::reference(Type::Path(call_struct.ty_path.clone()), None);
    let stmt = Stmt::Local(local::create_with_type(
        CALL_VAR_IDENT.clone(),
        call_var_type,
        LocalInit {
            eq_token: Default::default(),
            expr: Box::new(Expr::MethodCall(expr_method_call::create(
                vec![DYN_CALL_ARG_IDENT.clone()],
                constants::DYN_CALL_DOWNCAST_REF_FN_IDENT.clone(),
                Vec::new(),
            ))),
            diverge: None,
        },
    ));
    return stmt;
}

fn generate_check_stmt(call_struct: &CallStruct, skipped_fields_count: usize) -> Stmt {
    let check_exprs: Punctuated<_, Token![,]> = call_struct
        .item_struct
        .fields
        .iter()
        .skip(skipped_fields_count)
        .map(|field| generate_check_exprs(field))
        .collect();
    let vec_expr = Expr::Macro(ExprMacro {
        attrs: Vec::new(),
        mac: Macro {
            path: constants::MACRO_VEC_PATH.clone(),
            bang_token: Default::default(),
            delimiter: MacroDelimiter::Bracket(Bracket::default()),
            tokens: check_exprs.into_token_stream(),
        },
    });
    let stmt = Stmt::Expr(vec_expr, None);
    return stmt;
}

fn generate_check_exprs(field: &Field) -> Expr {
    let field_ident = field.get_required_ident();
    let receiver =
        field_access_expr::create(vec![constants::SELF_IDENT.clone(), field_ident.clone()]);
    let field_name_arg = str_lit::create_from_ident(&field_ident);
    let field_access_arg = expr_reference::create(field_access_expr::create(vec![
        CALL_VAR_IDENT.clone(),
        field_ident,
    ]));
    let field_string_value_arg = debug_string_expr::generate(field_access_arg.clone());
    let method = get_check_fn_ident(&field.ty);
    let expr = Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(receiver),
        dot_token: Default::default(),
        method,
        turbofish: None,
        paren_token: Default::default(),
        args: [field_name_arg, field_access_arg, field_string_value_arg]
            .into_iter()
            .collect(),
    });
    return expr;
}

fn get_check_fn_ident(ty: &Type) -> Ident {
    if let Type::Reference(_) = ty {
        return ARG_CHECK_REF_FN_IDENT.clone();
    }
    if let Type::Ptr(ptr) = ty
        && ptr.mutability.is_some()
    {
        return ARG_CHECK_MUT_REF_FN_IDENT.clone();
    }
    if let Type::Path(type_path) = ty {
        if let Some(ident) = type_path.path.segments.last().map(|x| &x.ident) {
            if ident == "Rc" {
                return ARG_CHECK_RC_FN_IDENT.clone();
            }
            if ident == "Arc" {
                return ARG_CHECK_ARC_FN_IDENT.clone();
            }
        }
    }
    return ARG_CHECK_FN_IDENT.clone();
}
