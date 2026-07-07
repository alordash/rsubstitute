use crate::common::models::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::common::*;
use crate::generation::mock_struct::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    mock_struct_path: Path,
    fn_info: &FnInfo,
    maybe_base_fn_ident: Option<Ident>,
) -> Block {
    let generic_arguments = generic_arguments::new(ctx, span, mock_struct_path.clone(), fn_info);
    let (fn_data_var_path, fn_data_stmt) =
        fn_data_stmt::new_associated(span, fn_info, generic_arguments);
    let fn_handle_stmt = fn_handle_stmt::generate(
        span,
        fn_handle_stmt::Params {
            mock_struct_path,
            fn_info,
            base_fn_kind: maybe_base_fn_ident
                .map(BaseFnKind::Associated)
                .unwrap_or(BaseFnKind::None),
            fn_data_var_path,
            is_static: false,
        },
    );

    let result = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(fn_data_stmt),
            Stmt::Expr(Expr::MethodCall(fn_handle_stmt), None),
        ],
    };
    return result;
}
