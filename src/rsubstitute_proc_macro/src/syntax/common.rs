use crate::syntax::r#type;
use proc_macro2::Span;
use syn::*;

pub(crate) fn self_type(span: Span) -> TypePath {
    let result = r#type::path::new(["Self"], span);

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
