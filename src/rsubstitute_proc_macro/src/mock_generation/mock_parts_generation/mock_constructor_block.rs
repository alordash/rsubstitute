use crate::constants;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use quote::format_ident;
use std::cell::LazyCell;
use syn::*;

pub(crate) fn generate(
    mock_struct: &MockStruct,
    mock_data_struct: &MockDataStruct,
    mock_setup_struct: &MockSetupStruct,
    mock_received_struct: &MockReceivedStruct,
    mock_struct_traits: Vec<&MockStructTrait>,
    maybe_inner_data_param: Option<InnerDataParam>,
) -> Block {
    let data_fn_fields: Vec<_> = mock_data_struct
        .field_and_fn_idents
        .iter()
        .map(|(field_ident, fn_ident)| {
            let func = path::create_expr_from_parts(vec![
                constants::FN_DATA_TYPE_IDENT.clone(),
                constants::NEW_IDENT.clone(),
            ]);
            let args = vec![str_lit::create(&fn_ident),];
            FieldValue {
                attrs: Vec::new(),
                member: Member::Named(field_ident.clone()),
                colon_token: Some(Default::default()),
                expr: expr_call::create_with_args(func, args),
            }
        })
        .collect();
    let data_fields = mock_data_struct
        .item_struct
        .fields
        .iter()
        .take(mock_data_struct.item_struct.fields.len() - data_fn_fields.len())
        .map(|field| FieldValue {
            attrs: Vec::new(),
            member: Member::Named(field.get_required_ident()),
            colon_token: Some(Default::default()),
            expr: constants::PHANTOM_DATA_EXPR_PATH.clone(),
        })
        .chain(data_fn_fields)
        .collect();
    let func = path::create_expr_from_parts(vec![
        constants::ARC_IDENT.clone(),
        constants::NEW_IDENT.clone(),
    ]);
    let arg = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: path::create(mock_data_struct.item_struct.ident.clone()),
        brace_token: Default::default(),
        fields: data_fields,
        dot2_token: None,
        rest: None,
    });
    let data_stmt = Stmt::Local(Local {
        attrs: Vec::new(),
        let_token: Default::default(),
        pat: Pat::Ident(PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            mutability: None,
            ident: DATA_VAR_IDENT.clone(),
            subpat: None,
        }),
        init: Some(LocalInit {
            eq_token: Default::default(),
            expr: Box::new(expr_call::create(func, arg)),
            diverge: None,
        }),
        semi_token: Default::default(),
    });
    let returned_type_path = path::create(mock_struct.item_struct.ident.clone());
    let mut return_stmt_fields = vec![
        FieldValue {
            attrs: Vec::new(),
            member: constants::SETUP_MEMBER.clone(),
            colon_token: Some(Default::default()),
            expr: Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: path::create(mock_setup_struct.item_struct.ident.clone()),
                brace_token: Default::default(),
                fields: [constants::DATA_FIELD_VALUE.clone()]
                    .into_iter()
                    .chain(mock_struct_traits.iter().map(|x| {
                        generate_mock_trait_setup_field(
                            x.info.trait_ident_from_path.clone(),
                            &x.setup_struct.item_struct,
                        )
                    }))
                    .collect(),
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
                path: path::create(mock_received_struct.item_struct.ident.clone()),
                brace_token: Default::default(),
                fields: [constants::DATA_FIELD_VALUE.clone()]
                    .into_iter()
                    .chain(mock_struct_traits.iter().map(|x| {
                        generate_mock_trait_setup_field(
                            x.info.trait_ident_from_path.clone(),
                            &x.received_struct.item_struct,
                        )
                    }))
                    .collect(),
                dot2_token: None,
                rest: None,
            }),
        },
        constants::DATA_SHORT_FIELD_VALUE.clone(),
    ];
    if maybe_inner_data_param.is_some() {
        let inner_data_field = FieldValue {
            attrs: Vec::new(),
            member: Member::Named(constants::INNER_DATA_FIELD_IDENT.clone()),
            colon_token: None,
            expr: constants::EMPTY_PATH_EXPR.clone(),
        };
        return_stmt_fields.push(inner_data_field);
    };
    let return_stmt = Stmt::Expr(
        Expr::Return(ExprReturn {
            attrs: Vec::new(),
            return_token: Default::default(),
            expr: Some(Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: returned_type_path,
                brace_token: Default::default(),
                fields: return_stmt_fields.into_iter().collect(),
                dot2_token: None,
                rest: None,
            }))),
        }),
        Some(Default::default()),
    );
    let stmts = if let Some(inner_data_param) = maybe_inner_data_param {
        let inner_data_stmt = generate_inner_data_stmt(inner_data_param);
        vec![data_stmt, inner_data_stmt, return_stmt]
    } else {
        vec![data_stmt, return_stmt]
    };
    let block = Block {
        brace_token: Default::default(),
        stmts,
    };
    return block;
}

const DATA_VAR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("data"));

fn generate_mock_trait_setup_field(
    trait_ident: Ident,
    mock_struct_trait_struct: &ItemStruct,
) -> FieldValue {
    let field_value = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(implemented_trait_ident::format_for_field(&trait_ident)),
        colon_token: Some(Default::default()),
        expr: Expr::Struct(ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: path::create(mock_struct_trait_struct.ident.clone()),
            brace_token: Default::default(),
            fields: [constants::DATA_FIELD_VALUE.clone()].into_iter().collect(),
            dot2_token: None,
            rest: None,
        }),
    };
    return field_value;
}

fn generate_inner_data_stmt(inner_data_param: InnerDataParam) -> Stmt {
    let func = path::create_expr_from_parts(vec![
        inner_data_param.inner_data_struct.item_struct.ident.clone(),
        constants::NEW_IDENT.clone(),
    ]);
    let args = inner_data_param
        .constructor_arguments
        .iter()
        .map(|constructor_argument| path::create_expr(constructor_argument.0.clone()))
        .collect();
    let local_init = LocalInit {
        eq_token: Default::default(),
        expr: Box::new(expr_call::create_with_args(func, args)),
        diverge: None,
    };
    let local = local::create(constants::INNER_DATA_FIELD_IDENT.clone(), local_init);
    let stmt = Stmt::Local(local);
    return stmt;
}
