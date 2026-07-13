use crate::generation::mock_controls::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(span: Span) -> ItemTrait {
    let type_mock = associated_type(span, "Mock");
    let fn_mock = TraitItemFn {
        attrs: Vec::new(),
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new("mock", span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: punctuated([self_fn_arg(span)]),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Path(r#type::path::new(span, ["Self", "Mock"]))),
            ),
        },
        default: None,
        semi_token: Some(Token![;](span)),
    };
    let type_static_setup = associated_type(span, "StaticSetup");
    let fn_static_setup = control_fn(span, ControlType::Setup);
    let type_static_received = associated_type(span, "StaticReceived");
    let fn_static_received = control_fn(span, ControlType::Received);

    let result = ItemTrait {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        unsafety: None,
        auto_token: None,
        restriction: None,
        trait_token: Token![trait](span),
        ident: Ident::new("Mockable", span),
        generics: Generics::default(),
        colon_token: None,
        supertraits: Punctuated::new(),
        brace_token: token::Brace(span),
        items: vec![
            TraitItem::Type(type_mock),
            TraitItem::Fn(fn_mock),
            TraitItem::Type(type_static_setup),
            TraitItem::Fn(fn_static_setup),
            TraitItem::Type(type_static_received),
            TraitItem::Fn(fn_static_received),
        ],
    };
    return result;
}

fn associated_type(span: Span, name: &'static str) -> TraitItemType {
    let result = TraitItemType {
        attrs: Vec::new(),
        type_token: Token![type](span),
        ident: Ident::new(name, span),
        generics: Generics::default(),
        colon_token: None,
        bounds: Punctuated::new(),
        default: None,
        semi_token: Token![;](span),
    };
    return result;
}

fn control_fn(span: Span, control_type: ControlType) -> TraitItemFn {
    let (ident_str, return_type_str) = match control_type {
        ControlType::Setup => ("static_setup", "StaticSetup"),
        ControlType::Received => ("static_received", "StaticReceived"),
    };
    let result = TraitItemFn {
        attrs: Vec::new(),
        sig: Signature {
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
                Box::new(Type::Path(r#type::path::new(
                    span,
                    ["Self", return_type_str],
                ))),
            ),
        },
        default: None,
        semi_token: Some(Token![;](span)),
    };
    return result;
}
