use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::*;

pub mod const_param;
pub mod lifetime_param;
pub mod type_param;

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

pub(crate) fn to_ident(generic_param: &GenericParam) -> Ident {
    let result = match generic_param {
        GenericParam::Lifetime(lifetime_param) => lifetime_param::to_ident(lifetime_param),
        GenericParam::Type(type_param) => type_param::to_ident(type_param),
        GenericParam::Const(const_param) => const_param::to_ident(const_param),
    };
    return result;
}
