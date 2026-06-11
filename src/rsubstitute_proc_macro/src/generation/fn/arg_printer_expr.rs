use crate::generation::r#fn::transmute_lifetime_expr;
use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn new(span: Span, expr: Expr, target_type: Type) -> Expr {
    let arg_printer = Expr::Call(expr::call::new(
        span,
        Expr::Path(expr::path::new(span, ["ArgPrinter"])),
        [transmute_lifetime_expr::new(expr, target_type)],
    ));
    let arg_printer_ref = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](span),
        mutability: None,
        expr: Box::new(arg_printer),
    });
    let arg_printer_ref_paren = Expr::Paren(ExprParen {
        attrs: Vec::new(),
        paren_token: token::Paren(span),
        expr: Box::new(arg_printer_ref),
    });
    let result = Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(arg_printer_ref_paren),
        dot_token: Token![.](span),
        method: Ident::new("debug_string", span),
        turbofish: None,
        paren_token: token::Paren(span),
        args: Punctuated::new(),
    });

    return result;
}
