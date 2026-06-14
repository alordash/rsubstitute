use crate::generation::mock_controls::constants;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(source_span: Span, target_type: Type) -> ItemImpl {
    let fn_clone = generate_fn_clone(source_span);
    let items = vec![ImplItem::Fn(fn_clone)];

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](source_span),
        generics: Generics::default(),
        trait_: Some((
            None,
            path::new(source_span, ["Clone"]),
            Token![for](source_span),
        )),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(source_span),
        items,
    };

    return result;
}

fn generate_fn_clone(source_span: Span) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](source_span),
        ident: Ident::new("clone", source_span),
        generics: Generics::default(),
        paren_token: token::Paren(source_span),
        inputs: punctuated([ref_self_fn_arg(source_span)]),
        variadic: None,
        output: ReturnType::Type(
            Token!(->)(source_span),
            Box::new(Type::Path(self_type(source_span))),
        ),
    };

    let self_struct_expr = ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: self_type_path(source_span),
        brace_token: token::Brace(source_span),
        fields: punctuated([FieldValue {
            attrs: Vec::new(),
            member: Member::Named(Ident::new(constants::DATA_FIELD, source_span)),
            colon_token: Some(Token![:](source_span)),
            expr: Expr::MethodCall(expr::method_call::new(
                source_span,
                Expr::Paren(ExprParen {
                    attrs: Vec::new(),
                    paren_token: token::Paren(source_span),
                    expr: Box::new(Expr::Reference(ExprReference {
                        attrs: Vec::new(),
                        and_token: Token![&](source_span),
                        mutability: None,
                        expr: Box::new(Expr::Field(expr::field::new(
                            Expr::Path(self_expr_path(source_span)),
                            Ident::new(constants::DATA_FIELD, source_span),
                        ))),
                    })),
                }),
                Ident::new("clone", source_span),
                [],
            )),
        }]),
        dot2_token: None,
        rest: None,
    };
    let self_stmt = Stmt::Expr(Expr::Struct(self_struct_expr), None);
    let block = Block {
        brace_token: token::Brace(source_span),
        stmts: vec![self_stmt],
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
