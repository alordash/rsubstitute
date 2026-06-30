use crate::common::rsubstitute_lifetime;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn new(mut generics: Generics) -> Generics {
    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: rsubstitute_lifetime::new(generics.span()),
            colon_token: None,
            bounds: Punctuated::new(),
        }),
    );
    return generics;
}
