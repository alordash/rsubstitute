use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span, fn_info: &FnInfo) -> (ExprPath, Local) {
    let fn_data_var_path = expr::path::new(span, ["call"]);
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Path(fn_data_var_path.clone()),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: fn_info.call_struct.path.clone(),
                brace_token: token::Brace(span),
                fields: [generics_field::new_value(span)]
                    .into_iter()
                    .chain(fn_info.arguments.iter().map(|x| FieldValue {
                        attrs: Vec::new(),
                        member: Member::Named(x.ident.clone()),
                        colon_token: Some(Token![:](span)),
                        expr: Expr::Macro(transmute_lifetime_expr::new(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: path::from_ident(x.ident.clone()),
                        }))),
                    }))
                    .collect(),
                dot2_token: None,
                rest: None,
            })),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    return (fn_data_var_path, fn_data_stmt);
}
