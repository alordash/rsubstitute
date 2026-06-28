use crate::generation::fn_info::models::*;
use crate::generation::{generics_field_value, transmute_lifetime_expr};
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    source_span: Span,
    fn_info: FnInfo,
    mock_struct_path: Path,
    base_fn_ident: Ident,
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
                        segments: [PathSegment {
                            ident: Ident::new("FnData", source_span),
                            arguments: PathArguments::AngleBracketed(
                                AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Token![<](source_span),
                                    args: [
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
                                    ]
                                    .into_iter()
                                    .collect(),
                                    gt_token: Token![>](source_span),
                                },
                            ),
                        }]
                        .into_iter()
                        .collect(),
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
    let handle_stmt = Expr::MethodCall(expr::method_call::new(
        source_span,
        Expr::Path(data_path),
        Ident::new("handle", source_span),
        [
            Expr::Reference(ExprReference {
                attrs: Vec::new(),
                and_token: Token![&](source_span),
                mutability: None,
                expr: Box::new(Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: mock_struct_path,
                    brace_token: token::Brace(source_span),
                    fields: [generics_field_value::new(source_span)]
                        .into_iter()
                        .collect(),
                    dot2_token: None,
                    rest: None,
                })),
            }),
            Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: fn_info.call_struct.path,
                brace_token: token::Brace(source_span),
                fields: [generics_field_value::new(source_span)]
                    .into_iter()
                    .chain(
                        fn_info
                            .syntax
                            .arguments
                            .into_iter()
                            .map(|argument| FieldValue {
                                attrs: Vec::new(),
                                member: Member::Named(argument.ident.clone()),
                                colon_token: Some(Token![:](source_span)),
                                expr: Expr::Macro(transmute_lifetime_expr::new(Expr::Path(
                                    ExprPath {
                                        attrs: Vec::new(),
                                        qself: None,
                                        path: path::from_ident(argument.ident),
                                    },
                                ))),
                            }),
                    )
                    .collect(),
                dot2_token: None,
                rest: None,
            }),
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
                                .into_iter()
                                .map(generic_argument::from_param)
                                .collect(),
                            gt_token: Token![>](source_span),
                        }),
                    }]
                    .into_iter()
                    .collect(),
                },
            }),
        ],
    ));

    let block = Block {
        brace_token: token::Brace(source_span),
        stmts: vec![Stmt::Local(data_stmt), Stmt::Expr(handle_stmt, None)],
    };

    let result = ItemFn {
        attrs: fn_info.syntax.attributes,
        vis: Visibility::Public(Token![pub](source_span)),
        sig: *fn_info.syntax.source_signature,
        block: Box::new(block),
    };
    return result;
}
