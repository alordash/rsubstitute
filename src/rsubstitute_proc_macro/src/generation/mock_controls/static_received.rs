use crate::common::models::*;
use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params<'a, 'b, 'c> {
    pub ctx: &'a Context,
    pub source_span: Span,
    pub target_ident: Ident,
    pub target_generics: Generics,
    pub mock_path: &'b Path,
    pub fn_infos: &'c [FnInfo],
    pub static_no_other_calls: bool,
}
pub(crate) fn generate(
    Params {
        ctx,
        source_span,
        target_ident,
        target_generics,
        mock_path,
        fn_infos,
        static_no_other_calls,
    }: Params,
) -> StaticReceivedStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](source_span)),
        struct_token: Token![struct](source_span),
        ident: format_ident!("{}StaticReceived", target_ident),
        generics: target_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(source_span),
            named: punctuated([generics_field::new_field(
                source_span,
                target_generics.clone(),
            )]),
        }),
        semi_token: None,
    };
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);

    let item_impl = generate_item_impl(
        ctx,
        source_span,
        target_generics,
        mock_path,
        path.clone(),
        fn_infos,
        static_no_other_calls,
    );

    let result = StaticReceivedStruct {
        path,
        item_struct,
        item_impl,
    };
    return result;
}

fn generate_item_impl(
    ctx: &Context,
    span: Span,
    target_generics: Generics,
    mock_path: &Path,
    static_received_struct_path: Path,
    fn_infos: &[FnInfo],
    static_no_other_calls: bool,
) -> ItemImpl {
    let items = fn_infos
        .iter()
        .map(|fn_info| generate_received_fn(ctx, span, mock_path, fn_info))
        .chain(core::iter::once(if static_no_other_calls {
            generate_fn_no_other_calls_for_static(ctx, span, mock_path.clone(), fn_infos)
        } else {
            generate_fn_no_other_calls_for_method(span, fn_infos)
        }))
        .map(ImplItem::Fn)
        .collect();

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: target_generics,
        trait_: None,
        self_ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: static_received_struct_path,
        })),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

fn generate_received_fn(
    ctx: &Context,
    span: Span,
    mock_path: &Path,
    fn_info: &FnInfo,
) -> ImplItemFn {
    let (times_arg_path, times_arg) = times_arg::new(span);
    let generic_arguments = generic_arguments::new(ctx, span, mock_path.clone(), fn_info);
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("received", span),
        generics: generics_with_rsubstitute_anonymous_lifetime::new(Generics::default()),
        paren_token: token::Paren(span),
        inputs: [self_fn_arg(span)]
            .into_iter()
            .chain(
                fn_info
                    .syntax
                    .arguments
                    .iter()
                    .map(|x| x.control_fn_arg.clone()),
            )
            .chain(core::iter::once(FnArg::Typed(times_arg)))
            .collect(),
        variadic: None,
        output: ReturnType::Type(Token![->](span), Box::new(Type::Path(self_type(span)))),
    };

    let (data_var_path, data_stmt) = fn_data_stmt::new_static(span, fn_info, generic_arguments);
    let (args_checker_var_path, args_checker_stmt) = args_checker_stmt::new(span, fn_info);
    let verify_received_stmt = Expr::MethodCall(expr::method_call::new(
        span,
        Expr::Path(data_var_path),
        Ident::new("verify_received", span),
        [
            Expr::Path(args_checker_var_path),
            Expr::Path(times_arg_path),
        ],
    ));
    let return_stmt = Expr::Path(self_expr_path(span));

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(data_stmt),
            Stmt::Local(args_checker_stmt),
            Stmt::Expr(verify_received_stmt, Some(Token![;](span))),
            Stmt::Expr(return_stmt, None),
        ],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        defaultness: None,
        sig,
        block,
    };
    return result;
}

fn generate_fn_no_other_calls_for_static(
    ctx: &Context,
    span: Span,
    mock_path: Path,
    fn_infos: &[FnInfo],
) -> ImplItemFn {
    let sig = fn_no_other_calls_signature(span);
    let fn_info = &fn_infos[0];
    let generic_arguments = generic_arguments::new(ctx, span, mock_path.clone(), fn_info);
    let (data_var_path, data_stmt) = fn_data_stmt::new_static(span, fn_info, generic_arguments);
    let verify_received_nothing_else_stmt = Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Path(data_var_path)),
        dot_token: Token![.](span),
        method: Ident::new("verify_received_nothing_else", span),
        turbofish: None,
        paren_token: token::Paren(span),
        args: punctuated([Expr::Array(ExprArray {
            attrs: Vec::new(),
            bracket_token: token::Bracket(span),
            elems: Punctuated::new(),
        })]),
    });

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(data_stmt),
            Stmt::Expr(verify_received_nothing_else_stmt, None),
        ],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        defaultness: None,
        sig,
        block,
    };
    return result;
}

fn generate_fn_no_other_calls_for_method(span: Span, fn_infos: &[FnInfo]) -> ImplItemFn {
    let sig = fn_no_other_calls_signature(span);
    let verify_received_nothing_else_stmt = Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Field(expr::field::new_self(Ident::new("data", span)))),
        dot_token: Token![.](span),
        method: Ident::new("verify_received_nothing_else", span),
        turbofish: None,
        paren_token: token::Paren(span),
        args: punctuated([Expr::Array(ExprArray {
            attrs: Vec::new(),
            bracket_token: token::Bracket(span),
            elems: fn_infos
                .iter()
                .map(|x| {
                    Expr::Lit(ExprLit {
                        attrs: Vec::new(),
                        lit: Lit::Str(LitStr::new(&x.syntax.fn_ident.to_string(), span)),
                    })
                })
                .collect(),
        })]),
    });

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(verify_received_nothing_else_stmt, None)],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        defaultness: None,
        sig,
        block,
    };
    return result;
}

fn fn_no_other_calls_signature(span: Span) -> Signature {
    let result = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("no_other_calls", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Default,
    };
    return result;
}
