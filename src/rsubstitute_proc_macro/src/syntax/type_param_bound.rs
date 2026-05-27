use crate::syntax::*;
use quote::ToTokens;
use syn::*;

pub mod precise_capture;
pub mod trait_bound;

pub(crate) fn to_ident(type_param_bound: &TypeParamBound) -> Ident {
    let result = match type_param_bound {
        TypeParamBound::Trait(trait_bound) => trait_bound::to_ident(trait_bound),
        TypeParamBound::Lifetime(lifetime) => lifetime::to_ident(lifetime),
        TypeParamBound::PreciseCapture(precise_capture) => {
            precise_capture::to_ident(precise_capture)
        }
        TypeParamBound::Verbatim(verbatim) => {
            panic!("Verbatim type parameter bound is not supported: {verbatim}")
        }
        unsupported => panic!(
            "Unsupported type parameter bound: {}",
            unsupported.to_token_stream().to_string()
        ),
    };
    return result;
}
