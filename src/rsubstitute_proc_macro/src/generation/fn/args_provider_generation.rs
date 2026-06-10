use crate::preparation::r#fn::models::*;
use crate::syntax::r#type::vec_of;
use crate::syntax::*;
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_args_provider_impl(
    arguments: &[Argument],
    target_type: Type,
    span: Span,
) -> ItemImpl {
    let fn_get_arg_infos = generate_fn_get_args_infos(arguments, span);
    let fn_get_ptr_to_boxed_tuple_of_refs = todo!();
    let items = vec![ImplItem::Fn(
        fn_get_arg_infos,
        fn_get_ptr_to_boxed_tuple_of_refs,
    )];

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: Generics::default(),
        trait_: Some((None, path::new(["IArgsProvider"], span), Token![for](span))),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}

fn generate_fn_get_args_infos(arguments: &[Argument], span: Span) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("get_arg_infos", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: [ref_self_fn_arg(span)].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(
            Token!(->)(span),
            Box::new(Type::Path(vec_of(
                Type::Path(r#type::path::new(["ArgInfo"], span)),
                span,
            ))),
        ),
    };

    let arg_info_new_exprs: Punctuated<Expr, Token![,]> =
        arguments.iter().map(generate_arg_info_new_expr).collect();
    let vec_stmt = Stmt::Expr(
        Expr::Macro(ExprMacro {
            attrs: Vec::new(),
            mac: r#macro::vec(arg_info_new_exprs.to_token_stream(), span),
        }),
        None,
    );
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![vec_stmt],
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

fn generate_arg_info_new_expr(argument: &Argument) -> Expr {
    let span = argument.ident.span();

    let arg_name_argument = Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: argument.ident.clone(),
                arguments: PathArguments::None,
            }]
            .into_iter()
            .collect(),
        },
    });

    let arg_field_expr = Expr::Field(ExprField {
        attrs: Vec::new(),
        base: Box::new(Expr::Path(self_path(span))),
        dot_token: Token![.](span),
        member: Member::Named(argument.ident.clone()),
    });
    let arg_value_argument = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](span),
        mutability: None,
        expr: Box::new(arg_field_expr.clone()),
    });

    let arg_debug_string_argument = Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Call(expr::call::new(
            Expr::Path(expr::path::new_generics(
                ["ArgPrinter"],
                GenericArgument::Type(*argument.pat_type.ty.clone()),
                span,
            )),
            [transmute_expr(arg_field_expr)],
            span,
        ))),
        dot_token: Token![.](span),
        method: Ident::new("debug_string", span),
        turbofish: None,
        paren_token: token::Paren(span),
        args: Punctuated::new(),
    });

    let result = Expr::Call(expr::call::new(
        Expr::Path(expr::path::new(["ArgInfo", "new"], span)),
        [
            arg_name_argument,
            arg_value_argument,
            arg_debug_string_argument,
        ],
        span,
    ));

    return result;
}
