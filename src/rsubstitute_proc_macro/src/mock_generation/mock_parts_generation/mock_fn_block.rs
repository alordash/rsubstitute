use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use quote::format_ident;
use std::cell::LazyCell;
use syn::*;

pub(crate) fn generate_for_trait(fn_info: &FnInfo) -> Block {
    let call_stmt = generate_call_stmt(fn_info);
    let last_stmts = generate_last_stmts(fn_info, Target::Other, None);
    let stmts = [call_stmt].into_iter().chain(last_stmts).collect();
    let block = Block {
        brace_token: Default::default(),
        stmts,
    };
    return block;
}

pub(crate) fn generate_for_struct_trait_fn(fn_info: &FnInfo, trait_ident: &Ident) -> Block {
    let call_stmt = generate_call_stmt(fn_info);
    let last_stmts = generate_last_stmts(fn_info, Target::Other, Some(trait_ident));
    let stmts = [call_stmt].into_iter().chain(last_stmts).collect();
    let block = Block {
        brace_token: Default::default(),
        stmts,
    };
    return block;
}

pub(crate) fn generate_for_static(fn_info: &FnInfo, mock_type: &MockType) -> Block {
    let call_stmt = generate_call_stmt(fn_info);
    let last_stmts = generate_last_stmts(fn_info, Target::StaticFn(mock_type), None);
    let stmts = [call_stmt].into_iter().chain(last_stmts).collect();
    let block = Block {
        brace_token: Default::default(),
        stmts,
    };
    return block;
}

const CALL_VARIABLE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));
const HANDLE_METHOD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("handle"));
const HANDLE_RETURNING_METHOD_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("handle_returning"));
const HANDLE_BASE_METHOD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("handle_base"));
const HANDLE_BASE_RETURNING_METHOD_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("handle_base_returning"));
const MOCK_VARIABLE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("mock"));

fn generate_call_stmt(fn_info: &FnInfo) -> Stmt {
    let field_values: Vec<_> = fn_info
        .call_struct
        .item_struct
        .fields
        .iter()
        .map(|field| {
            let field_ident = field.get_required_ident();
            if field::is_phantom_data(field) {
                return field_value::create_as_phantom_data(field_ident);
            }
            let field_value = FieldValue {
                attrs: Vec::new(),
                member: Member::Named(field_ident.clone()),
                colon_token: Some(Default::default()),
                expr: transmute_lifetime_expr::create(field_ident),
            };
            return field_value;
        })
        .collect();
    let mut call_struct_type_generics = fn_info.call_struct.item_struct.generics.clone();
    let Some(GenericParam::Lifetime(first_lifetime_param)) =
        call_struct_type_generics.params.first_mut()
    else {
        panic!("Call struct should have default lifetime as first generics parameter");
    };
    first_lifetime_param.lifetime = constants::ANONYMOUS_LIFETIME.clone();
    let mut call_struct_type = r#type::create_with_generics(
        fn_info.call_struct.item_struct.ident.clone(),
        call_struct_type_generics,
    );
    reference::set_all_lifetimes(&mut call_struct_type, &constants::ANONYMOUS_LIFETIME.clone());
    let call_stmt = Stmt::Local(local::create_with_type(
        CALL_VARIABLE_IDENT.clone(),
        call_struct_type,
        LocalInit {
            eq_token: Default::default(),
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: path::create(fn_info.call_struct.item_struct.ident.clone()),
                brace_token: Default::default(),
                fields: field_values.into_iter().collect(),
                dot2_token: None,
                rest: None,
            })),
            diverge: None,
        },
    ));
    return call_stmt;
}

fn generate_last_stmts(
    fn_info: &FnInfo,
    target: Target,
    maybe_containing_trait_ident: Option<&Ident>,
) -> Vec<Stmt> {
    let base_receiver = path::create_expr(match target {
        Target::Other => constants::SELF_IDENT.clone(),
        Target::StaticFn(_) => MOCK_VARIABLE_IDENT.clone(),
    });
    let mut handle_expr =
        generate_handle_expr(fn_info, base_receiver, target, maybe_containing_trait_ident);
    if fn_info.parent.has_return_value() {
        handle_expr = Expr::Return(ExprReturn {
            attrs: Vec::new(),
            return_token: Default::default(),
            expr: Some(Box::new(handle_expr)),
        });
    }
    let handle_stmt = Stmt::Expr(handle_expr, Some(Default::default()));
    let last_stmts = match target {
        Target::Other => vec![handle_stmt],
        Target::StaticFn(mock_type) => {
            let mock_var_stmt = Stmt::Local(local::create(
                MOCK_VARIABLE_IDENT.clone(),
                LocalInit {
                    eq_token: Default::default(),
                    expr: Box::new(get_global_mock_expr::generate(mock_type.ty.clone())),
                    diverge: None,
                },
            ));
            vec![mock_var_stmt, handle_stmt]
        }
    };
    return last_stmts;
}

fn generate_handle_expr(
    fn_info: &FnInfo,
    base_receiver: Expr,
    target: Target,
    maybe_containing_trait_ident: Option<&Ident>,
) -> Expr {
    let idents = vec![
        constants::DATA_IDENT.clone(),
        fn_info.data_field_ident.clone(),
    ];
    let method_name = match (
        fn_info.parent.maybe_base_fn_block.is_some(),
        fn_info.parent.has_return_value(),
    ) {
        (false, false) => HANDLE_METHOD_IDENT.clone(),
        (false, true) => HANDLE_RETURNING_METHOD_IDENT.clone(),
        (true, false) => HANDLE_BASE_METHOD_IDENT.clone(),
        (true, true) => HANDLE_BASE_RETURNING_METHOD_IDENT.clone(),
    };
    let args = if fn_info.parent.maybe_base_fn_block.is_some() {
        let specific_fn_info_ident = match maybe_containing_trait_ident {
            None => &fn_info.parent.fn_ident,
            Some(containing_trait_ident) => {
                &format_ident!("{}_{}", containing_trait_ident, fn_info.parent.fn_ident)
            }
        };
        let base_fn_ident = base_fn_ident::format(specific_fn_info_ident);
        let base_fn_path = match target {
            Target::StaticFn(_) => path::create_expr(base_fn_ident),
            Target::Other => path::create_expr_from_parts(vec![
                constants::SELF_TYPE_IDENT.clone(),
                base_fn_ident,
            ]),
        };
        vec![
            expr_reference::create(base_receiver.clone()),
            path::create_expr(CALL_VARIABLE_IDENT.clone()),
            base_fn_path,
        ]
    } else {
        vec![
            expr_reference::create(base_receiver.clone()),
            path::create_expr(CALL_VARIABLE_IDENT.clone()),
        ]
    };
    let expr_method_call = expr_method_call::create_with_base_receiver_and_expr_args(
        base_receiver,
        idents,
        method_name,
        args,
    );
    let expr = Expr::MethodCall(expr_method_call);
    return expr;
}

#[derive(Clone, Copy)]
enum Target<'a> {
    Other,
    StaticFn(&'a MockType),
}
