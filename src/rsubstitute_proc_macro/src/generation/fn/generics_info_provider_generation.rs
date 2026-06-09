use crate::syntax::*;
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_generics_info_provider_impl(
    generics: Generics,
    target_type: Type,
) -> ItemImpl {
    let span = generics.span();
    let fn_get_generic_parameter_infos =
        generate_fn_get_generic_parameter_infos(generics.params.iter(), span);
    let fn_hash_generics_type_ids =
        generate_fn_hash_generics_type_ids(generics.type_params(), span);
    let fn_hash_const_values = generate_fn_hash_const_values(generics.const_params(), span);
    let items = vec![
        ImplItem::Fn(fn_get_generic_parameter_infos),
        ImplItem::Fn(fn_hash_generics_type_ids),
        ImplItem::Fn(fn_hash_const_values),
    ];

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: Some((
            None,
            // todo - maybe somehow test that it's equal to real trait
            path::new(["IGenericsInfoProvider"], span),
            Token![for](span),
        )),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

fn generate_fn_get_generic_parameter_infos<'a>(
    generic_params: impl Iterator<Item = &'a GenericParam>,
    span: Span,
) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("get_generic_parameter_infos", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: [ref_self_fn_arg(span)].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(r#type::vec_of(
                Type::Path(r#type::path::new(["GenericParameterInfo"], span)),
                span,
            ))),
        ),
    };

    let generic_parameter_infos: Punctuated<Expr, Token![,]> = generic_params
        .filter_map(|generic_param| match generic_param {
            GenericParam::Type(type_param) => Some(Expr::Call(expr::call::new(
                Expr::Path(expr::path::new(["generic_type_info"], span)),
                [
                    Expr::Path(expr::path::new([&type_param.ident.to_string()], span)),
                    Expr::Call(expr::call::new(
                        Expr::Path(expr::path::new(["core", "any", "type_name"], span)),
                        [],
                        span,
                    )),
                ],
                span,
            ))),
            GenericParam::Const(const_param) => {
                let const_param_ident_string = const_param.ident.to_string();
                Some(Expr::Call(expr::call::new(
                    Expr::Path(expr::path::new(["generic_const_info"], span)),
                    [
                        Expr::Lit(ExprLit {
                            attrs: Vec::new(),
                            lit: Lit::Str(LitStr::new(&const_param_ident_string, span)),
                        }),
                        Expr::Path(expr::path::new([&const_param_ident_string], span)),
                    ],
                    span,
                )))
            }
            _ => None,
        })
        .collect();

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Macro(StmtMacro {
            attrs: Vec::new(),
            mac: Macro {
                path: path::new(["vec"], span),
                bang_token: Token![!](span),
                delimiter: MacroDelimiter::Bracket(token::Bracket(span)),
                tokens: generic_parameter_infos.to_token_stream(),
            },
            semi_token: None,
        })],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };
    return result;
}

fn generate_fn_hash_generics_type_ids<'a>(
    type_params: impl Iterator<Item = &'a TypeParam>,
    span: Span,
) -> ImplItemFn {
    let sig = generate_hash_fn_sig("hash_generics_type_ids", span);
    let tids: Punctuated<_, _> = type_params
        .into_iter()
        .map(|type_param| {
            Expr::Call(expr::call::new(
                Expr::Path(expr::path::new_generics(
                    ["tid"],
                    GenericArgument::Type(Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [PathSegment {
                                ident: type_param.ident.clone(),
                                arguments: PathArguments::None,
                            }]
                            .into_iter()
                            .collect(),
                        },
                    })),
                    span,
                )),
                [],
                span,
            ))
        })
        .collect();
    let stmts = if tids.len() > 0 {
        let tids_array = Expr::Array(ExprArray {
            attrs: Vec::new(),
            bracket_token: token::Bracket(span),
            elems: tids,
        });
        let stmt = Stmt::Expr(tids_array, None);
        vec![stmt]
    } else {
        Vec::new()
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block: Block {
            brace_token: token::Brace(span),
            stmts,
        },
    };
    return result;
}

fn generate_fn_hash_const_values<'a>(
    const_params: impl Iterator<Item = &'a ConstParam>,
    span: Span,
) -> ImplItemFn {
    let sig = generate_hash_fn_sig("hash_const_values", span);
    let stmts = const_params
        .map(|const_param| {
            let const_hash_expr = Expr::Call(expr::call::new(
                Expr::Path(expr::path::new(["const_hash"], span)),
                [
                    Expr::Reference(ExprReference {
                        attrs: Vec::new(),
                        and_token: Token![&](span),
                        mutability: None,
                        expr: Box::new(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [PathSegment {
                                    ident: const_param.ident.clone(),
                                    arguments: PathArguments::None,
                                }]
                                .into_iter()
                                .collect(),
                            },
                        })),
                    }),
                    Expr::Path(expr::path::new(["hasher"], span)),
                ],
                span,
            ));
            let stmt = Stmt::Expr(const_hash_expr, Some(Token![;](span)));
            return stmt;
        })
        .collect();

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block: Block {
            brace_token: token::Brace(span),
            stmts,
        },
    };
    return result;
}

fn generate_hash_fn_sig(fn_name: &'static str, span: Span) -> Signature {
    let inputs = [
        ref_self_fn_arg(span),
        FnArg::Typed(PatType {
            attrs: vec![attributes::allow_unused_variables(span)],
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: Ident::new("hasher", span),
                subpat: None,
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Reference(TypeReference {
                and_token: Token![&](span),
                lifetime: None,
                mutability: Some(Token![mut](span)),
                elem: Box::new(Type::Path(r#type::path::new(["GenericsHasher"], span))),
            })),
        }),
    ]
    .into_iter()
    .collect();

    let result = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new(fn_name, span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs,
        variadic: None,
        output: ReturnType::Default,
    };
    return result;
}
