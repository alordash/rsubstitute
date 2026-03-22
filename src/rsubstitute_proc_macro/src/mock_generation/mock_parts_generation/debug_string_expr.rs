use crate::constants;
use crate::syntax::*;
use syn::*;

pub(crate) fn generate(expr: Expr, maybe_actual_source_type: Option<&Type>) -> Expr {
    let mut argument = reference::create_expr(expr);
    let arg_printer_path = if let Some(mut actual_source_type) = maybe_actual_source_type.cloned() {
        lifetime::remove_all_lifetimes(&mut actual_source_type);
        argument = transmute_lifetime_expr::create_for_expr(argument);
        path::create_expr_with_generic_type(
            constants::ARG_PRINTER_STRUCT_IDENT.clone(),
            actual_source_type,
        )
    } else {
        path::create_expr(constants::ARG_PRINTER_STRUCT_IDENT.clone())
    };
    let method_call_expr = Expr::MethodCall(method_call::create_with_base_receiver(
        Expr::Paren(ExprParen {
            attrs: Vec::new(),
            paren_token: Default::default(),
            expr: Box::new(reference::create_expr(call::create(
                arg_printer_path,
                argument,
            ))),
        }),
        Vec::new(),
        constants::DEBUG_STRING_FN_IDENT.clone(),
        Vec::new(),
    ));
    return method_call_expr;
}
