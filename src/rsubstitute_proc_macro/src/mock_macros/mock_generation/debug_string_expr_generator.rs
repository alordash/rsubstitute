use crate::constants;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IDebugStringExprGenerator {
    fn generate(&self, expr: Expr) -> Expr;
}

pub(crate) struct DebugStringExprGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub expr_reference_factory: Arc<dyn IExprReferenceFactory>,
}

impl IDebugStringExprGenerator for DebugStringExprGenerator {
    fn generate(&self, expr: Expr) -> Expr {
        let method_call_expr = Expr::MethodCall(
            self.expr_method_call_factory.create_with_base_receiver(
                Expr::Paren(ExprParen {
                    attrs: Vec::new(),
                    paren_token: Default::default(),
                    expr: Box::new(
                        self.expr_reference_factory.create(Expr::Call(ExprCall {
                            attrs: Vec::new(),
                            func: Box::new(
                                self.path_factory
                                    .create_expr(constants::ARG_PRINTER_STRUCT_IDENT.clone()),
                            ),
                            paren_token: Default::default(),
                            args: [self.expr_reference_factory.create(expr)]
                                .into_iter()
                                .collect(),
                        })),
                    ),
                }),
                Vec::new(),
                constants::DEBUG_STRING_FN_IDENT.clone(),
                Vec::new(),
            ),
        );
        return method_call_expr;
    }
}
