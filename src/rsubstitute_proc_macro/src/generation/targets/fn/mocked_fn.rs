use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    source_span: Span,
    fn_info: &FnInfo,
    mock_struct_path: Path,
    maybe_base_fn_ident: Option<Ident>,
) -> ItemFn {
    let data_path = expr::path::new(source_span, ["data"]);
    let data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](source_span),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(data_path.clone())),
            colon_token: Token![:](source_span),
            ty: Box::new(Type::Reference(TypeReference {
                and_token: Token![&](source_span),
                lifetime: None,
                mutability: None,
                elem: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: punctuated([PathSegment {
                            ident: Ident::new("FnData", source_span),
                            arguments: PathArguments::AngleBracketed(
                                AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Token![<](source_span),
                                    args: punctuated([
                                        GenericArgument::Type(Type::Path(TypePath {
                                            qself: None,
                                            path: mock_struct_path.clone(),
                                        })),
                                        generic_argument::bool(
                                            source_span,
                                            match fn_info.syntax.source_signature.output {
                                                ReturnType::Type(_, _) => true,
                                                _ => false,
                                            },
                                        ),
                                        generic_argument::bool(source_span, true),
                                        generic_argument::bool(source_span, false),
                                    ]),
                                    gt_token: Token![>](source_span),
                                },
                            ),
                        }]),
                    },
                })),
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](source_span),
            expr: Box::new(Expr::Call(expr::call::new(
                source_span,
                Expr::Path(expr::path::new(source_span, ["get_static_fn_data"])),
                [Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Str(LitStr::new(
                        &fn_info.syntax.source_signature.ident.to_string(),
                        source_span,
                    )),
                })],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](source_span),
    };
    let mock_arg = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](source_span),
        mutability: None,
        expr: Box::new(Expr::Struct(ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: mock_struct_path,
            brace_token: token::Brace(source_span),
            fields: [generics_field::new_value(source_span)]
                .into_iter()
                .collect(),
            dot2_token: None,
            rest: None,
        })),
    });
    let the_call = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: fn_info.call_struct.path.clone(),
        brace_token: token::Brace(source_span),
        fields: [generics_field::new_value(source_span)]
            .into_iter()
            .chain(fn_info.syntax.arguments.iter().map(|x| FieldValue {
                attrs: Vec::new(),
                member: Member::Named(x.ident.clone()),
                colon_token: Some(Token![:](source_span)),
                expr: Expr::Macro(transmute_lifetime_expr::new(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::from_ident(x.ident.clone()),
                }))),
            }))
            .collect(),
        dot2_token: None,
        rest: None,
    });
    let maybe_base_call = maybe_base_fn_ident.map(|base_fn_ident| {
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: base_fn_ident,
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: Some(Token![::](source_span)),
                        lt_token: Token![<](source_span),
                        args: fn_info
                            .syntax
                            .merged_generics
                            .params
                            .iter()
                            .cloned()
                            .map(generic_argument::from_param)
                            .collect(),
                        gt_token: Token![>](source_span),
                    }),
                }]
                .into_iter()
                .collect(),
            },
        })
    });
    let args = if let Some(base_call) = maybe_base_call {
        [mock_arg, the_call, base_call].into_iter().collect()
    } else {
        [mock_arg, the_call].into_iter().collect()
    };
    let handle_stmt = Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Path(data_path)),
        dot_token: Token![.](source_span),
        method: Ident::new("handle", source_span),
        turbofish: None,
        paren_token: token::Paren(source_span),
        args,
    });

    let block = Block {
        brace_token: token::Brace(source_span),
        stmts: vec![Stmt::Local(data_stmt), Stmt::Expr(handle_stmt, None)],
    };

    let result = ItemFn {
        attrs: fn_info.syntax.attributes.clone(),
        vis: Visibility::Public(Token![pub](source_span)),
        sig: *fn_info.syntax.source_signature.clone(),
        block: Box::new(block),
    };
    return result;
}
