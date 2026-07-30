use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct AssociatedParams<'a> {
    pub fn_info: &'a FnInfo,
    pub generic_arguments: generic_arguments::Result,
    pub generics_info_provider_var_path: ExprPath,
    pub for_struct: bool,
}
pub(crate) fn new_associated(
    span: Span,
    AssociatedParams {
        fn_info,
        generic_arguments,
        generics_info_provider_var_path,
        for_struct,
    }: AssociatedParams,
) -> (ExprPath, Local) {
    let fn_data_var_path = expr::path::new(span, ["fn_data"]);
    let fn_name = if for_struct {
        "get_shared_fn_data_for_struct"
    } else {
        "get_shared_fn_data"
    };
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: fn_data_pat(span, fn_data_var_path.clone(), generic_arguments),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::MethodCall(expr::method_call::new(
                span,
                Expr::Field(expr::field::new_self(Ident::new("data", span))),
                Ident::new(fn_name, span),
                [
                    fn_info_ident_to_expr_lit(span, fn_info),
                    Expr::MethodCall(expr::method_call::new(
                        span,
                        Expr::Path(generics_info_provider_var_path),
                        Ident::new("get_generics_hash_key", span),
                        [],
                    )),
                ],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    return (fn_data_var_path, fn_data_stmt);
}

pub(crate) struct StaticParams<'a> {
    pub fn_info: &'a FnInfo,
    pub generic_arguments: generic_arguments::Result,
    pub for_struct: bool,
}
pub(crate) fn new_static(
    span: Span,
    StaticParams {
        fn_info,
        generic_arguments,
        for_struct,
    }: StaticParams,
) -> (ExprPath, Local) {
    let fn_data_var_path = expr::path::new(span, ["fn_data"]);
    let fn_name = if for_struct {
        "get_static_fn_data_for_struct"
    } else {
        "get_static_fn_data"
    };
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: fn_data_pat(span, fn_data_var_path.clone(), generic_arguments),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Call(expr::call::new(
                span,
                Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::new_global(span, ["rsubstitute", "for_generated", fn_name]),
                }),
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
        lit: Lit::Str(LitStr::new(&fn_info.fn_data_name, span)),
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
            attrs: Vec::new(),
            and_token: Token![&](span),
            lifetime: None,
            mutability: None,
            elem: Box::new(Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: Some(Token![::](span)),
                    segments: punctuated([
                        PathSegment {
                            ident: Ident::new("rsubstitute", span),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: Ident::new("for_generated", span),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: Ident::new("FnData", span),
                            arguments: PathArguments::AngleBracketed(
                                AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Token![<](span),
                                    args: punctuated([
                                        generic_arguments.mock_generic_argument,
                                        generic_arguments.has_return_value_argument,
                                        generic_arguments.supports_base_calling_argument,
                                        generic_arguments.passes_mock_to_callback_argument,
                                    ]),
                                    gt_token: Token![>](span),
                                },
                            ),
                        },
                    ]),
                },
            })),
        })),
    })
}
