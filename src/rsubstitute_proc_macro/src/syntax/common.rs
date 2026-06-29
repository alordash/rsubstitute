use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

const SELF: &'static str = "self";
const SELF_TYPE: &'static str = "Self";

pub(crate) fn self_path(span: Span) -> Path {
    let result = path::new(span, [SELF]);
    return result;
}

pub(crate) fn self_expr_path(span: Span) -> ExprPath {
    let result = expr::path::new(span, [SELF]);
    return result;
}

pub(crate) fn self_type_path(span: Span) -> Path {
    let result = path::new(span, [SELF_TYPE]);
    return result;
}

pub(crate) fn self_type(span: Span) -> TypePath {
    let result = r#type::path::new(span, [SELF_TYPE]);
    return result;
}

pub(crate) fn ref_self_type(span: Span) -> TypeReference {
    let result = TypeReference {
        and_token: Token![&](span),
        lifetime: None,
        mutability: None,
        elem: Box::new(Type::Path(self_type(span))),
    };
    return result;
}

pub(crate) fn self_fn_arg(span: Span) -> FnArg {
    let result = FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        reference: None,
        mutability: None,
        self_token: Token![self](Span::call_site()),
        colon_token: None,
        ty: Box::new(Type::Path(self_type(span))),
    });
    return result;
}

pub(crate) fn ref_self_fn_arg(span: Span) -> FnArg {
    let result = FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        reference: Some((Token![&](Span::call_site()), None)),
        mutability: None,
        self_token: Token![self](Span::call_site()),
        colon_token: None,
        ty: Box::new(Type::Reference(ref_self_type(span))),
    });
    return result;
}

pub(crate) fn void_type(span: Span) -> Type {
    let result = TypeTuple {
        paren_token: token::Paren(span),
        elems: Punctuated::new(),
    };
    return Type::Tuple(result);
}

pub(crate) fn mut_ptr_infer_type(span: Span) -> Type {
    let result = Type::Ptr(TypePtr {
        star_token: Token![*](span),
        const_token: None,
        mutability: Some(Token![mut](span)),
        elem: Box::new(Type::Infer(TypeInfer {
            underscore_token: Token![_](span),
        })),
    });
    return result;
}

pub(crate) fn mut_ptr_void_type(span: Span) -> Type {
    let result = Type::Ptr(TypePtr {
        star_token: Token![*](span),
        const_token: None,
        mutability: Some(Token![mut](span)),
        elem: Box::new(void_type(span)),
    });
    return result;
}

pub(crate) fn static_lifetime(span: Span) -> Lifetime {
    let result = Lifetime {
        apostrophe: span,
        ident: Ident::new("static", span),
    };
    return result;
}

pub(crate) fn generics_field_ident(span: Span) -> Ident {
    let result = Ident::new("generics", span);
    return result;
}

pub(crate) fn placeholder_lifetime(span: Span) -> Lifetime {
    let result = Lifetime {
        apostrophe: span,
        ident: Ident::new("_", span),
    };
    return result;
}

pub(crate) fn punctuated<T, P: Default, const N: usize>(items: [T; N]) -> Punctuated<T, P> {
    items.into_iter().collect()
}
