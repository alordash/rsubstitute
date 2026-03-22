use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use syn::*;

pub(crate) fn generate(
    fn_info: &FnInfo,
    mock_received_struct: &MockReceivedStruct,
    mock_type: &MockType,
) -> ItemFn {
    let sig = received_signature::generate_for_static(fn_info, mock_received_struct, mock_type);
    let block = generate_fn_received_block(fn_info, mock_type);
    let item_fn = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Default::default()),
        sig,
        block: Box::new(block),
    };
    return item_fn;
}

fn generate_fn_received_block(fn_info: &FnInfo, mock_type: &MockType) -> Block {
    let static_mock_expr = constants::GET_MOCK_FN_CALL_EXPR.clone();
    let received_clone_expr = Expr::MethodCall(method_call::create_with_base_receiver(
        static_mock_expr,
        vec![constants::MOCK_RECEIVED_FIELD_IDENT.clone()],
        constants::CLONE_FN_IDENT.clone(),
        Vec::new(),
    ));
    let return_stmt = Stmt::Expr(
        Expr::Return(ExprReturn {
            attrs: Vec::new(),
            return_token: Default::default(),
            expr: Some(Box::new(Expr::MethodCall(
                method_call::create_with_base_receiver(
                    received_clone_expr,
                    Vec::new(),
                    constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
                    fn_info
                        .args_checker_struct
                        .item_struct
                        .fields
                        .iter()
                        .skip(
                            fn_info.parent.get_internal_phantom_types_count()
                                + mock_type.generics.get_phantom_fields_count(),
                        )
                        .map(IFieldExtensions::get_required_ident)
                        .chain(std::iter::once(received_signature::get_times_arg_ident()))
                        .collect(),
                ),
            ))),
        }),
        Some(Default::default()),
    );
    let stmts = vec![return_stmt];
    let block = Block {
        brace_token: Default::default(),
        stmts,
    };
    return block;
}
