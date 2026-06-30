use crate::common::generics_field;
use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::common::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    mock_path: Path,
    static_setup_path: Path,
    fn_info: &FnInfo,
) -> ItemFn {
    let generic_arguments = generic_arguments::new(ctx, span, mock_path, fn_info);
    let fn_configurator_path = fn_configurator_path::new(
        span,
        fn_info,
        &generic_arguments,
        static_lifetime(span),
        Some(Type::Path(TypePath {
            qself: None,
            path: static_setup_path.clone(),
        })),
    );

    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("setup", span),
        generics: generics_with_rsubstitute_anonymous_lifetime::new(
            fn_info.syntax.merged_generics.clone(),
        ),
        paren_token: token::Paren(span),
        inputs: fn_info
            .syntax
            .arguments
            .iter()
            .map(|x| x.control_fn_arg.clone())
            .collect(),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(TypePath {
                qself: None,
                path: fn_configurator_path,
            })),
        ),
    };

    let (data_var_path, data_stmt) = fn_data_stmt::new_static(span, fn_info, generic_arguments);
    let data_reset_stmt = expr::method_call::new(
        span,
        Expr::Path(data_var_path),
        Ident::new("reset", span),
        [],
    );
    let setup_stmt = ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Struct(ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: static_setup_path,
            brace_token: token::Brace(span),
            fields: punctuated([generics_field::new_value(span)]),
            dot2_token: None,
            rest: None,
        })),
        dot_token: Token![.](span),
        method: Ident::new("setup", span),
        turbofish: None,
        paren_token: token::Paren(span),
        args: fn_info
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
    };

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(data_stmt),
            Stmt::Expr(Expr::MethodCall(data_reset_stmt), Some(Token![;](span))),
            Stmt::Expr(Expr::MethodCall(setup_stmt), None),
        ],
    };

    let result = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        sig,
        block: Box::new(block),
    };
    return result;
}
