use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

const SELF: &'static str = "self";
const SELF_TYPE: &'static str = "Self";

pub(crate) fn self_expr_path(span: Span) -> ExprPath {
    expr::path::new(span, [SELF])
}

pub(crate) fn self_type_ident(span: Span) -> Ident {
    Ident::new(SELF_TYPE, span)
}

pub(crate) fn self_type_path(span: Span) -> Path {
    path::new(span, [SELF_TYPE])
}

pub(crate) fn self_type(span: Span) -> TypePath {
    r#type::path::new(span, [SELF_TYPE])
}

pub(crate) fn ref_self_fn_arg(span: Span) -> FnArg {
    FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        mutability: None,
        self_token: Token![self](Span::call_site()),
        kind: ReceiverKind::Reference(Token![&](span), None, None),
    })
}

pub(crate) fn mut_ref_self_fn_arg(span: Span) -> FnArg {
    FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        mutability: Some(Token![mut](span)),
        self_token: Token![self](Span::call_site()),
        kind: ReceiverKind::Reference(Token![&](span), None, Some(Token![mut](span))),
    })
}

pub(crate) fn void_type(span: Span) -> Type {
    Type::Tuple(TypeTuple {
        attrs: Vec::new(),
        paren_token: token::Paren(span),
        elems: Punctuated::new(),
    })
}

pub(crate) fn void_tuple(span: Span) -> Expr {
    Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: token::Paren(span),
        elems: Punctuated::new(),
    })
}

pub(crate) fn mut_ptr_infer_type(span: Span) -> Type {
    Type::Ptr(TypePtr {
        attrs: Vec::new(),
        star_token: Token![*](span),
        mutability: PointerMutability::Mut(Token![mut](span)),
        elem: Box::new(Type::Infer(TypeInfer {
            attrs: Vec::new(),
            underscore_token: Token![_](span),
        })),
    })
}

pub(crate) fn mut_ptr_void_type(span: Span) -> Type {
    Type::Ptr(TypePtr {
        attrs: Vec::new(),
        star_token: Token![*](span),
        mutability: PointerMutability::Mut(Token![mut](span)),
        elem: Box::new(void_type(span)),
    })
}

pub(crate) fn generics_field_ident(span: Span) -> Ident {
    Ident::new("__rs_generics", span)
}

pub(crate) fn placeholder_lifetime(span: Span) -> Lifetime {
    Lifetime {
        apostrophe: span,
        ident: Ident::new("_", span),
    }
}

pub(crate) fn punctuated<T, P: Default, const N: usize>(items: [T; N]) -> Punctuated<T, P> {
    items.into_iter().collect()
}

pub(crate) fn anonymous_lifetime_generic_argument(span: Span) -> GenericArgument {
    GenericArgument::Lifetime(Lifetime::new("'_", span))
}