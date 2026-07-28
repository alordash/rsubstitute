use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn new(span: Span) -> Lifetime {
    let result = Lifetime::new("'__rsa", span);

    return result;
}

pub(crate) fn prepend_to_generics(mut generics: Generics) -> Generics {
    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: new(generics.span()),
            colon_token: None,
            bounds: Punctuated::new(),
        }),
    );
    return generics;
}
