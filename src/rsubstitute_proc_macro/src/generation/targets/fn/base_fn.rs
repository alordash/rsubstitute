use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::preparation::r#fn::models::IArgumentTypesCloner;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(
    source_span: Span,
    fn_info: &FnInfo,
    mock_struct_path: Path,
    base_impl: Box<Block>,
) -> ItemFn {
    let source_signature = &fn_info.syntax.source_signature;
    let call_path = path::new(source_span, ["call"]);
    let sig = Signature {
        constness: source_signature.constness.clone(),
        asyncness: source_signature.asyncness.clone(),
        unsafety: source_signature.unsafety.clone(),
        abi: source_signature.abi.clone(),
        fn_token: Token![fn](source_span),
        ident: format_ident!("__rs_base_{}", source_signature.ident),
        generics: source_signature.generics.clone(),
        paren_token: token::Paren(source_span),
        inputs: punctuated([
            FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Wild(PatWild {
                    attrs: Vec::new(),
                    underscore_token: Token![_](source_span),
                })),
                colon_token: Token![:](source_span),
                ty: Box::new(Type::Reference(TypeReference {
                    and_token: Token![&](source_span),
                    lifetime: None,
                    mutability: None,
                    elem: Box::new(Type::Path(TypePath {
                        qself: None,
                        path: mock_struct_path,
                    })),
                })),
            }),
            FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: call_path.clone(),
                })),
                colon_token: Token![:](source_span),
                ty: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: fn_info.call_struct.path.clone(),
                })),
            }),
        ]),
        variadic: None,
        output: source_signature.output.clone(),
    };

    let deconstruct_call_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](source_span),
        pat: Pat::Struct(PatStruct {
            attrs: Vec::new(),
            qself: None,
            path: fn_info.call_struct.path.clone(),
            brace_token: token::Brace(source_span),
            fields: fn_info
                .syntax
                .arguments
                .iter()
                .map(|x| FieldPat {
                    attrs: Vec::new(),
                    member: Member::Named(x.ident.clone()),
                    colon_token: Some(Token![:](source_span)),
                    pat: Box::new(Pat::Path(PatPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::from_ident(x.ident.clone()),
                    })),
                })
                .collect(),
            rest: Some(PatRest {
                attrs: Vec::new(),
                dot2_token: Token![..](source_span),
            }),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](source_span),
            expr: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: call_path,
            })),
            diverge: None,
        }),
        semi_token: Token![;](source_span),
    };
    let cast_args_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](source_span),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Tuple(PatTuple {
                attrs: Vec::new(),
                paren_token: token::Paren(source_span),
                elems: fn_info
                    .syntax
                    .arguments
                    .iter()
                    .map(|x| *x.source_pat_type.pat.clone())
                    .collect(),
            })),
            colon_token: Token![:](source_span),
            ty: Box::new(Type::Tuple(TypeTuple {
                paren_token: token::Paren(source_span),
                elems: fn_info.syntax.arguments.iter_generics_style_types().collect(),
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](source_span),
            expr: Box::new(Expr::Macro(transmute_lifetime_expr::new(Expr::Tuple(
                ExprTuple {
                    attrs: Vec::new(),
                    paren_token: token::Paren(source_span),
                    elems: fn_info
                        .syntax
                        .arguments
                        .iter()
                        .map(|x| {
                            Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: path::from_ident(x.ident.clone()),
                            })
                        })
                        .collect(),
                },
            )))),
            diverge: None,
        }),
        semi_token: Token![;](source_span),
    };

    let block = Block {
        brace_token: token::Brace(source_span),
        stmts: vec![
            Stmt::Local(deconstruct_call_stmt),
            Stmt::Local(cast_args_stmt),
            Stmt::Expr(
                Expr::Block(ExprBlock {
                    attrs: Vec::new(),
                    label: None,
                    block: *base_impl,
                }),
                None,
            ),
        ],
    };

    let result = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        sig,
        block: Box::new(block),
    };
    return result;
}
