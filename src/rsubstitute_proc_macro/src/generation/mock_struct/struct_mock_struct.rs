use crate::common::generics_field;
use crate::generation::common::data_field;
use crate::generation::mock_struct::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params {
    pub struct_ident: Ident,
    pub struct_mock_ident: Ident,
    pub generics: Generics,
    pub struct_setup_ident: Ident,
    pub struct_received_ident: Ident,
}
pub(crate) fn generate(
    span: Span,
    Params {
        struct_ident,
        struct_mock_ident,
        generics,
        struct_setup_ident,
        struct_received_ident,
    }: Params,
) -> StructMockStruct {
    let path = path::from_ident_with_generics(struct_ident.clone(), &generics);
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: struct_mock_ident,
        generics: generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([
                generics_field::new_field(span, generics, None),
                data_field::new_field(span, data_field::Params { public: true }),
                Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(Ident::new("mockable", span)),
                    colon_token: Some(Token![:](span)),
                    ty: Type::Path(r#type::box_of(
                        span,
                        Type::Path(TypePath {
                            qself: None,
                            path: path.clone(),
                        }),
                    )),
                },
            ]),
        }),
        semi_token: None,
    };
    let struct_type = Type::Path(TypePath {
        qself: None,
        path: path::from_ident_with_generics(struct_ident, &item_struct.generics),
    });
    let struct_mock_type = Type::Path(TypePath {
        qself: None,
        path: path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics),
    });
    let deref_mut_impl = deref_impl(
        span,
        struct_type.clone(),
        struct_mock_type.clone(),
        item_struct.generics.clone(),
        true,
    );
    let deref_impl = deref_impl(
        span,
        struct_type,
        struct_mock_type,
        item_struct.generics.clone(),
        false,
    );
    let item_impl = item_impl(
        span,
        Type::Path(TypePath {
            qself: None,
            path: path.clone(),
        }),
        Type::Path(TypePath {
            qself: None,
            path: path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics),
        }),
        item_struct.generics.clone(),
        struct_setup_ident,
        struct_received_ident,
    );
    let result = StructMockStruct {
        path,
        item_struct,
        item_impl,
        deref_impl,
        deref_mut_impl,
    };
    return result;
}

fn item_impl(
    span: Span,
    struct_type: Type,
    struct_mock_type: Type,
    generics: Generics,
    struct_setup_ident: Ident,
    struct_received_ident: Ident,
) -> ItemImpl {
    let fn_unmock = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        defaultness: None,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new("unmock", span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: punctuated([self_fn_arg(span)]),
            variadic: None,
            output: ReturnType::Type(Token![->](span), Box::new(struct_type)),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(
                Expr::Unary(ExprUnary {
                    attrs: Vec::new(),
                    op: UnOp::Deref(Token![*](span)),
                    expr: Box::new(Expr::Field(expr::field::new_self(Ident::new(
                        "mockable", span,
                    )))),
                }),
                None,
            )],
        },
    };
    let fn_setup = fn_control(
        span,
        ControlTarget::Setup(path::from_ident_with_generics(
            struct_setup_ident,
            &generics,
        )),
    );
    let fn_received = fn_control(
        span,
        ControlTarget::Received(path::from_ident_with_generics(
            struct_received_ident,
            &generics,
        )),
    );
    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: generics.clone(),
        trait_: None,
        self_ty: Box::new(struct_mock_type),
        brace_token: token::Brace(span),
        items: vec![
            ImplItem::Fn(fn_unmock),
            ImplItem::Fn(fn_setup),
            ImplItem::Fn(fn_received),
        ],
    };
    return result;
}

enum ControlTarget {
    Setup(Path),
    Received(Path),
}

fn fn_control(span: Span, control_target: ControlTarget) -> ImplItemFn {
    let (fn_name, return_type_path) = match control_target {
        ControlTarget::Setup(control_path) => ("setup", control_path),
        ControlTarget::Received(control_path) => ("received", control_path),
    };
    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        defaultness: None,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new(fn_name, span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: punctuated([self_fn_arg(span)]),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Path(TypePath {
                    qself: None,
                    path: return_type_path.clone(),
                })),
            ),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(
                Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: return_type_path,
                    brace_token: token::Brace(span),
                    fields: punctuated([
                        generics_field::new_value(span),
                        data_field::new_clone_value(span),
                    ]),
                    dot2_token: None,
                    rest: None,
                }),
                None,
            )],
        },
    };
    return result;
}

fn deref_impl(
    span: Span,
    struct_type: Type,
    struct_mock_type: Type,
    generics: Generics,
    r#mut: bool,
) -> ItemImpl {
    let (fn_name, trait_name, deref_fn_name) = if r#mut {
        ("deref_mut", "DerefMut", "as_mut")
    } else {
        ("deref", "Deref", "as_ref")
    };
    let fn_deref = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new(fn_name, span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: punctuated([if r#mut {
                mut_ref_self_fn_arg(span)
            } else {
                ref_self_fn_arg(span)
            }]),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Reference(TypeReference {
                    and_token: Token![&](span),
                    lifetime: None,
                    mutability: r#mut.then(|| Token![mut](span)),
                    elem: Box::new(Type::Path(r#type::path::new(span, ["Self", "Target"]))),
                })),
            ),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(
                Expr::MethodCall(expr::method_call::new(
                    span,
                    Expr::Field(expr::field::new_self(Ident::new("mockable", span))),
                    Ident::new(deref_fn_name, span),
                    [],
                )),
                None,
            )],
        },
    };
    let items = if r#mut {
        vec![ImplItem::Fn(fn_deref)]
    } else {
        vec![
            ImplItem::Type(ImplItemType {
                attrs: Vec::new(),
                vis: Visibility::Inherited,
                defaultness: None,
                type_token: Token![type](span),
                ident: Ident::new("Target", span),
                generics: Generics::default(),
                eq_token: Token![=](span),
                ty: struct_type,
                semi_token: Token![;](span),
            }),
            ImplItem::Fn(fn_deref),
        ]
    };
    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: generics.clone(),
        trait_: Some((
            None,
            path::new(span, ["core", "ops", trait_name]),
            Token![for](span),
        )),
        self_ty: Box::new(struct_mock_type),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}
