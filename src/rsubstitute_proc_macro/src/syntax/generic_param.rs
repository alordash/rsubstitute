use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn from_type_ident(type_ident: Ident) -> GenericParam {
    let result = GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ident: type_ident,
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    });
    return result;
}
