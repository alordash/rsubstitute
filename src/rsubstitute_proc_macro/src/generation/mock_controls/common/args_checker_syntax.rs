use crate::generation::r#fn::models::*;
use crate::generation::transmute_lifetime_expr;
use crate::syntax::{expr, path};
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) struct Result {
    pub local: Local,
    pub var_path: Path,
}

pub(crate) fn new(span: Span, fn_info: &FnInfo) -> Result {
    let var_path = path::from_ident(format_ident!("{}_args_checker", fn_info.syntax.fn_ident));

    let local = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: var_path.clone(),
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Path(TypePath {
                qself: None,
                path: fn_info.args_checker_struct.path.clone(),
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: fn_info.args_checker_struct.path.clone(),
                brace_token: token::Brace(span),
                fields: fn_info
                    .syntax
                    .arguments
                    .iter()
                    .map(|argument| FieldValue {
                        attrs: Vec::new(),
                        member: Member::Named(argument.ident.clone()),
                        colon_token: Some(Token![:](span)),
                        expr: Expr::Macro(transmute_lifetime_expr::new(Expr::MethodCall(
                            expr::method_call::new(
                                span,
                                Expr::Path(ExprPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: path::from_ident(argument.ident.clone()),
                                }),
                                Ident::new("into", span),
                                [],
                            ),
                        ))),
                    })
                    .collect(),
                dot2_token: None,
                rest: None,
            })),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };

    let result = Result { local, var_path };

    return result;
}
