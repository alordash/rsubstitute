use crate::generation::r#fn::transmute_lifetime_expr;
use crate::preparation::r#fn::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate_args_checker_impl(
    span: Span,
    arguments: &[Argument],
    target_type: Type,
    call_struct_type: Type,
) -> ItemImpl {
    let fn_check = generate_fn_check(span, arguments, call_struct_type);
    let fn_fmt_args = generate_fn_fmt_args(span, arguments);
    let items = vec![ImplItem::Fn(fn_check), ImplItem::Fn(fn_fmt_args)];

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: Generics::default(),
        trait_: Some((None, path::new(span, ["IArgsChecker"]), Token![for](span))),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}

fn generate_fn_check(span: Span, arguments: &[Argument], call_struct_type: Type) -> ImplItemFn {
    let dyn_call_arg_path = path::new(span, ["dyn_call"]);
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("check", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: [
            ref_self_fn_arg(span),
            FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Path(PatPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: dyn_call_arg_path.clone(),
                })),
                colon_token: Token![:](span),
                ty: Box::new(Type::Reference(TypeReference {
                    and_token: Token![&](span),
                    lifetime: None,
                    mutability: None,
                    elem: Box::new(Type::Path(r#type::path::new(span, ["DynCall"]))),
                })),
            }),
        ]
        .into_iter()
        .collect(),
        variadic: None,
        output: ReturnType::Type(
            Token!(->)(span),
            Box::new(Type::Path(r#type::vec_of(
                span,
                Type::Path(r#type::path::new(span, ["ArgCheckResult"])),
            ))),
        ),
    };

    let call_stmt = Stmt::Local(Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(PatPath {
                attrs: Vec::new(),
                qself: None,
                path: path::new(span, ["span"]),
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Reference(TypeReference {
                and_token: Token![&](span),
                lifetime: None,
                mutability: None,
                elem: Box::new(call_struct_type),
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::MethodCall(expr::method_call::new(
                span,
                Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: dyn_call_arg_path,
                }),
                Ident::new("downcast_ref", span),
                [],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    });
    let vec_stmt_exprs: Punctuated<Expr, Token![,]> = arguments
        .iter()
        .map(|argument| {
            let result = expr::method_call::new(
                span,
                Expr::Macro(transmute_lifetime_expr::new_with_target(
                    Expr::Reference(ExprReference {
                        attrs: Vec::new(),
                        and_token: Token![&](span),
                        mutability: None,
                        expr: Box::new(Expr::Field(expr::field::new_self(argument.ident.clone()))),
                    }),
                    todo!(Arg<arg_type>),
                )),
                Ident::new("check_ref", argument.ident.span()),
                [],
            );
            return Expr::MethodCall(result);
        })
        .collect();
    let vec_stmt = todo!();
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![call_stmt, vec_stmt],
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

fn generate_fn_fmt_args(span: Span, arguments: &[Argument]) -> ImplItemFn {
    let result = todo!();

    return result;
}
