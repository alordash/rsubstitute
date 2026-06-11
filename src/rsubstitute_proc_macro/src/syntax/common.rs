use crate::syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn self_path(span: Span) -> ExprPath {
    let result = expr::path::new(span, ["self"]);
    return result;
}

pub(crate) fn self_type(span: Span) -> TypePath {
    let result = r#type::path::new(span, ["Self"]);
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

pub(crate) fn ref_self_fn_arg(span: Span) -> FnArg {
    let result = FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        reference: Some((Token![&](Span::call_site()), None)),
        mutability: None,
        self_token: Token!(self)(Span::call_site()),
        colon_token: None,
        ty: Box::new(Type::Reference(ref_self_type(span))),
    });

    return result;
}

pub(crate) fn mut_ptr_infer(span: Span) -> Type {
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

pub(crate) fn mut_ptr_void(span: Span) -> Type {
    let result = Type::Ptr(TypePtr {
        star_token: Token![*](span),
        const_token: None,
        mutability: Some(Token![mut](span)),
        elem: Box::new(Type::Tuple(TypeTuple {
            paren_token: token::Paren(span),
            elems: Punctuated::new(),
        })),
    });

    return result;
}
