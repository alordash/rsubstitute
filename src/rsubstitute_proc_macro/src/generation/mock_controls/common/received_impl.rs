use crate::common::models::*;
use crate::common::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::common::*;
use crate::syntax::*;
use proc_macro2::{Ident, Span};
use std::borrow::Borrow;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params<'a, T: Borrow<FnInfo>> {
    pub received_struct_path: Path,
    pub generics: Generics,
    pub mock_struct_path: &'a Path,
    pub fn_infos: &'a [T],
    pub for_static_fn: bool,
    pub is_static: bool,
}
pub(crate) fn generate<T: Borrow<FnInfo>>(
    ctx: &Context,
    span: Span,
    Params {
        received_struct_path,
        generics,
        mock_struct_path,
        fn_infos,
        for_static_fn,
        is_static,
    }: Params<T>,
) -> ItemImpl {
    let items = fn_infos
        .iter()
        .map(|fn_info| {
            generate_received_fn(ctx, span, mock_struct_path, fn_info.borrow(), for_static_fn)
        })
        .chain(core::iter::once(if is_static {
            generate_fn_no_other_calls_for_static_fn(ctx, span, mock_struct_path.clone(), fn_infos)
        } else {
            generate_regular_fn_no_other_calls(span, fn_infos)
        }))
        .map(ImplItem::Fn)
        .collect();

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: None,
        self_ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: received_struct_path,
        })),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

fn generate_received_fn(
    ctx: &Context,
    span: Span,
    mock_struct_path: &Path,
    fn_info: &FnInfo,
    for_static_fn: bool,
) -> ImplItemFn {
    let (times_arg_path, times_arg) = times_arg::new(span);
    let generic_arguments = generic_arguments::new(ctx, span, mock_struct_path.clone(), fn_info);
    let rsubstitute_lifetime = rsubstitute_lifetime::new(span);
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: if for_static_fn {
            Ident::new("received", span)
        } else {
            fn_info.source_signature.ident.clone()
        },
        generics: generics::with_prefix_lifetime(
            generics::with_lifetimes_tied_to(
                Generics::default(),
                &fn_info.merged_generics,
                rsubstitute_lifetime.clone(),
            ),
            rsubstitute_lifetime,
        ),
        paren_token: token::Paren(span),
        inputs: [self_fn_arg(span)]
            .into_iter()
            .chain(fn_info.arguments.iter().map(|x| x.control_fn_arg.clone()))
            .chain(core::iter::once(FnArg::Typed(times_arg)))
            .collect(),
        variadic: None,
        output: ReturnType::Type(Token![->](span), Box::new(Type::Path(self_type(span)))),
    };

    let (fn_data_var_path, fn_data_stmt) =
        fn_data_stmt::new_static(span, fn_info, generic_arguments);
    let (args_checker_var_path, args_checker_stmt) = args_checker_stmt::new(span, fn_info);
    let verify_received_stmt = Expr::MethodCall(expr::method_call::new(
        span,
        Expr::Path(fn_data_var_path),
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
            Stmt::Local(fn_data_stmt),
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

fn generate_fn_no_other_calls_for_static_fn<T: Borrow<FnInfo>>(
    ctx: &Context,
    span: Span,
    mock_struct_path: Path,
    fn_infos: &[T],
) -> ImplItemFn {
    let sig = fn_no_other_calls_signature(span);
    let fn_info = &fn_infos[0];
    let generic_arguments =
        generic_arguments::new(ctx, span, mock_struct_path.clone(), fn_info.borrow());
    let (fn_data_var_path, fn_data_stmt) =
        fn_data_stmt::new_static(span, fn_info.borrow(), generic_arguments);
    let verify_received_nothing_else_stmt = Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Path(fn_data_var_path)),
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
            Stmt::Local(fn_data_stmt),
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

fn generate_regular_fn_no_other_calls<T: Borrow<FnInfo>>(span: Span, fn_infos: &[T]) -> ImplItemFn {
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
                        lit: Lit::Str(LitStr::new(&x.borrow().fn_ident.to_string(), span)),
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
