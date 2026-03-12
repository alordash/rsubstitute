use crate::constants;
use crate::syntax::*;
use syn::*;

pub(crate) trait IDebugStringExprGenerator {
    fn generate(&self, expr: Expr) -> Expr;
}

pub(crate) struct DebugStringExprGenerator;

impl IDebugStringExprGenerator for DebugStringExprGenerator {
    fn generate(&self, expr: Expr) -> Expr {
        let method_call_expr = Expr::MethodCall(expr_method_call::create_with_base_receiver(
            Expr::Paren(ExprParen {
                attrs: Vec::new(),
                paren_token: Default::default(),
                expr: Box::new(expr_reference::create(expr_call::create(
                    path::create_expr(constants::ARG_PRINTER_STRUCT_IDENT.clone()),
                    expr_reference::create(expr),
                ))),
            }),
            Vec::new(),
            constants::DEBUG_STRING_FN_IDENT.clone(),
            Vec::new(),
        ));
        return method_call_expr;
    }
}
