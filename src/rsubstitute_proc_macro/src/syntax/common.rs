use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

const SELF: &'static str = "self";
const SELF_TYPE: &'static str = "Self";

pub(crate) fn self_path(span: Span) -> Path {
    path::new(span, [SELF])
}

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

pub(crate) fn ref_self_type(span: Span) -> TypeReference {
    TypeReference {
        attrs: Vec::new(),
        and_token: Token![&](span),
        lifetime: None,
        mutability: None,
        elem: Box::new(Type::Path(self_type(span))),
    }
}

pub(crate) fn mut_ref_self_type(span: Span) -> TypeReference {
    TypeReference {
        attrs: Vec::new(),
        and_token: Token![&](span),
        lifetime: None,
        mutability: Some(Token![mut](span)),
        elem: Box::new(Type::Path(self_type(span))),
    }
}

pub(crate) fn self_fn_arg() -> FnArg {
    FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        mutability: None,
        self_token: Token![self](Span::call_site()),
        kind: ReceiverKind::Value,
    })
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

pub(crate) fn static_lifetime(span: Span) -> Lifetime {
    Lifetime {
        apostrophe: span,
        ident: Ident::new("static", span),
    }
}

pub(crate) fn generics_field_ident(span: Span) -> Ident {
    Ident::new("generics", span)
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

pub(crate) fn rsubstitute_punctuated<P: Default, const N: usize>(
    span: Span,
    items: [PathSegment; N],
) -> Punctuated<PathSegment, P> {
    [
        PathSegment {
            ident: Ident::new("rsubstitute", span),
            arguments: PathArguments::None,
        },
        PathSegment {
            ident: Ident::new("for_generated", span),
            arguments: PathArguments::None,
        },
    ]
    .into_iter()
    .chain(items.into_iter())
    .collect()
}
