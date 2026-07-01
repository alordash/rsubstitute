use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(
    span: Span,
    fn_info: &FnInfo,
    generic_arguments: generic_arguments::Result,
) -> (ExprPath, Local) {
    let fn_data_var_path = expr::path::new(span, ["fn_data"]);
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: fn_data_pat(span, fn_data_var_path.clone(), generic_arguments),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::MethodCall(expr::method_call::new(
                span,
                Expr::Field(expr::field::new_self(Ident::new("data", span))),
                Ident::new("get_shared_fn_data", span),
                [fn_info_ident_to_expr_lit(span, fn_info)],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    return (fn_data_var_path, fn_data_stmt);
}

pub(crate) fn new_static(
    span: Span,
    fn_info: &FnInfo,
    generic_arguments: generic_arguments::Result,
) -> (ExprPath, Local) {
    let fn_data_var_path = expr::path::new(span, ["fn_data"]);
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: fn_data_pat(span, fn_data_var_path.clone(), generic_arguments),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new(span, ["get_static_fn_data"])),
                [fn_info_ident_to_expr_lit(span, fn_info)],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    return (fn_data_var_path, fn_data_stmt);
}

fn fn_info_ident_to_expr_lit(span: Span, fn_info: &FnInfo) -> Expr {
    Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Str(LitStr::new(&fn_info.syntax.fn_ident.to_string(), span)),
    })
}

fn fn_data_pat(
    span: Span,
    data_var_path: ExprPath,
    generic_arguments: generic_arguments::Result,
) -> Pat {
    Pat::Type(PatType {
        attrs: Vec::new(),
        pat: Box::new(Pat::Path(data_var_path.clone())),
        colon_token: Token![:](span),
        ty: Box::new(Type::Reference(TypeReference {
            and_token: Token![&](span),
            lifetime: None,
            mutability: None,
            elem: Box::new(Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: punctuated([PathSegment {
                        ident: Ident::new("FnData", span),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Token![<](span),
                            args: punctuated([
                                generic_arguments.mock_generic_argument,
                                generic_arguments.has_return_value_argument,
                                generic_arguments.supports_base_calling_argument,
                                generic_arguments.passes_mock_to_callback_argument,
                            ]),
                            gt_token: Token![>](span),
                        }),
                    }]),
                },
            })),
        })),
    })
}
