use crate::generation::r#fn::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(span: Span, fn_infos: &[FnInfo], target_type: Type) -> ItemImpl {
    let fn_get_received_nothing_else_error_msgs =
        generate_fn_get_received_nothing_else_error_msgs(span, fn_infos);

    let items = vec![ImplItem::Fn(fn_get_received_nothing_else_error_msgs)];

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: Generics::default(),
        trait_: Some((None, path::new(span, ["IMockData"]), Token![for](span))),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}

fn generate_fn_get_received_nothing_else_error_msgs(span: Span, fn_infos: &[FnInfo]) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("get_received_nothing_else_error_msgs", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Type(
            Token!(->)(span),
            Box::new(Type::Path(r#type::vec_of(
                span,
                Type::Path(r#type::vec_of(
                    span,
                    Type::Path(r#type::path::new(span, ["String"])),
                )),
            ))),
        ),
    };

    let vec_exprs: Punctuated<Expr, Token![,]> = fn_infos
        .iter()
        .map(|fn_info| {
            let span = fn_info.syntax.spans.inputs;
            let result = expr::method_call::new(
                span,
                Expr::Field(expr::field::new_self(fn_info.syntax.fn_ident.clone())),
                Ident::new("get_unexpected_calls_error_msgs", span),
                [],
            );

            return Expr::MethodCall(result);
        })
        .collect();
    let vec_expr = ExprMacro {
        attrs: Vec::new(),
        mac: r#macro::vec(span, vec_exprs.to_token_stream()),
    };

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(Expr::Macro(vec_expr), None)],
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
