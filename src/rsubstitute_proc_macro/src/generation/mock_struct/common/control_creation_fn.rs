use crate::common::generics_field;
use crate::generation::common::data_field;
use crate::generation::mock_controls::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate_associated(
    span: Span,
    control_path: Path,
    control_type: ControlType,
) -> ImplItemFn {
    let ident_str = match control_type {
        ControlType::Setup => "setup",
        ControlType::Received => "received",
    };

    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new(ident_str, span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([mut_ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(TypePath {
                qself: None,
                path: control_path.clone(),
            })),
        ),
    };
    let constructor_stmt = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: control_path,
        brace_token: token::Brace(span),
        fields: punctuated([data_field::new_clone_value(span)]),
        dot2_token: None,
        rest: None,
    });
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(constructor_stmt, None)],
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

pub(crate) fn generate_static(
    span: Span,
    control_path: Path,
    control_type: ControlType,
) -> ImplItemFn {
    let ident_str = match control_type {
        ControlType::Setup => "static_setup",
        ControlType::Received => "static_received",
    };
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new(ident_str, span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: Punctuated::new(),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(TypePath {
                qself: None,
                path: control_path.clone(),
            })),
        ),
    };
    let constructor_stmt = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: control_path,
        brace_token: token::Brace(span),
        fields: punctuated([generics_field::new_value(span)]),
        dot2_token: None,
        rest: None,
    });
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(constructor_stmt, None)],
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
