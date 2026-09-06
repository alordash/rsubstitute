use crate::common::*;
use crate::syntax::*;
use quote::format_ident;
use syn::*;

const ARGUMENT_ARG_NAME: &'static str = "v";

pub(crate) fn create(
    owning_function_signature: &Signature,
    argument_ident: Ident,
    argument_type: Box<Type>,
) -> ItemFn {
    let span = argument_ident.span();
    let use_stmt = Stmt::Item(Item::Use(rsubstitute_for_generated::glob_usage(
        span,
        "arg_printing",
    )));
    let arg_printer = Expr::Call(expr::call::new(
        span,
        Expr::Path(expr::path::new_global(
            span,
            rsubstitute_for_generated::new("ArgPrinter"),
        )),
        [Expr::Path(expr::path::new(span, [ARGUMENT_ARG_NAME]))],
    ));
    let arg_printer_ref = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](span),
        mutability: None,
        expr: Box::new(arg_printer),
    });
    let arg_printer_ref_paren = Expr::Paren(ExprParen {
        attrs: Vec::new(),
        paren_token: token::Paren(span),
        expr: Box::new(arg_printer_ref),
    });
    let arg_printer_expr = expr::method_call::new(
        span,
        arg_printer_ref_paren,
        Ident::new("debug_string", span),
        [],
    );
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            use_stmt,
            Stmt::Expr(Expr::MethodCall(arg_printer_expr), None),
        ],
    };
    let result = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig: Signature {
            constness: None,
            asyncness: None,
            safety: Safety::Default,
            abi: None,
            fn_token: Token![fn](span),
            ident: format_ident!("fmt_{}_{}", owning_function_signature.ident, argument_ident),
            generics: owning_function_signature.generics.clone(),
            paren_token: token::Paren(span),
            inputs: punctuated([FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Path(PatPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::new(span, [ARGUMENT_ARG_NAME]),
                })),
                colon_token: Token![:](span),
                ty: Box::new(Type::Reference(TypeReference {
                    attrs: Vec::new(),
                    and_token: Token![&](span),
                    lifetime: None,
                    mutability: None,
                    elem: argument_type,
                })),
            })]),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Path(r#type::path::new(span, ["String"]))),
            ),
        },
        block: Box::new(block),
    };
    return result;
}
