use crate::common::*;
use crate::generation::common::*;
use crate::generation::mock_controls::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params {
    pub struct_ident: Ident,
    pub generics: Generics,
    pub struct_mock_ident: Ident,
    pub static_setup_struct_ident: Ident,
    pub static_received_struct_ident: Ident,
}
pub(crate) fn generate(
    span: Span,
    Params {
        struct_ident,
        generics,
        struct_mock_ident,
        static_setup_struct_ident,
        static_received_struct_ident,
    }: Params,
) -> ItemImpl {
    let struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(struct_ident, &generics),
    });
    let struct_mock_path = path::from_ident_with_generics(struct_mock_ident.clone(), &generics);
    let static_setup_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(static_setup_struct_ident, &generics),
    });
    let static_received_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(static_received_struct_ident, &generics),
    });
    let struct_mock_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(struct_mock_ident, &generics),
    });
    let type_mock = associated_type_impl(span, "Mock", struct_mock_type);
    let fn_mock = fn_mock(span, struct_mock_path);
    let type_static_setup = associated_type_impl(span, "StaticSetup", static_setup_struct_type);
    let fn_static_setup = fn_static_control(span, ControlType::Setup);
    let type_static_received =
        associated_type_impl(span, "StaticReceived", static_received_struct_type);
    let fn_static_received = fn_static_control(span, ControlType::Received);
    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: Some((
            path::new_global(span, ["rsubstitute", "Mockable"]),
            Token![for](span),
        )),
        self_ty: Box::new(struct_type),
        brace_token: token::Brace(span),
        items: vec![
            ImplItem::Type(type_mock),
            ImplItem::Fn(fn_mock),
            ImplItem::Type(type_static_setup),
            ImplItem::Fn(fn_static_setup),
            ImplItem::Type(type_static_received),
            ImplItem::Fn(fn_static_received),
        ],
    };
    return result;
}

fn associated_type_impl(span: Span, name: &'static str, ty: Type) -> ImplItemType {
    let result = ImplItemType {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: TypeModifiers::default(),
        type_token: Token![type](span),
        ident: Ident::new(name, span),
        generics: Generics::default(),
        eq_token: Token![=](span),
        ty,
        semi_token: Token![;](span),
    };
    return result;
}

fn fn_mock(span: Span, struct_mock_path: Path) -> ImplItemFn {
    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig: Signature {
            constness: None,
            asyncness: None,
            safety: Safety::Default,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new("mock", span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: punctuated([self_fn_arg()]),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::new(span, ["Self", "Mock"]),
                })),
            ),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(
                Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: struct_mock_path,
                    brace_token: token::Brace(span),
                    fields: punctuated([
                        generics_field::new_value(span),
                        data_field::new_default_value(span),
                        FieldValue {
                            attrs: Vec::new(),
                            member: Member::Named(Ident::new("mockable", span)),
                            colon_token: Some(Token![:](span)),
                            expr: Expr::Call(expr::call::new(
                                span,
                                Expr::Path(ExprPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: path::new(span, ["Box", "new"]),
                                }),
                                [Expr::Path(self_expr_path(span))],
                            )),
                        },
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

fn fn_static_control(span: Span, control_type: ControlType) -> ImplItemFn {
    let (fn_name, static_control_name) = match control_type {
        ControlType::Setup => ("static_setup", "StaticSetup"),
        ControlType::Received => ("static_received", "StaticReceived"),
    };
    let static_control_path = path::new(span, ["Self", static_control_name]);
    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig: Signature {
            constness: None,
            asyncness: None,
            safety: Safety::Default,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new(fn_name, span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: Punctuated::new(),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: static_control_path.clone(),
                })),
            ),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(
                Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: static_control_path,
                    brace_token: token::Brace(span),
                    fields: punctuated([generics_field::new_value(span)]),
                    dot2_token: None,
                    rest: None,
                }),
                None,
            )],
        },
    };
    return result;
}
