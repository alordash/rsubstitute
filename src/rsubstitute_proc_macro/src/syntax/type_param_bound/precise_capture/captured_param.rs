use crate::syntax::lifetime;
use quote::ToTokens;
use syn::*;

pub(crate) fn to_ident(captured_param: &CapturedParam) -> Ident {
    let result = match captured_param {
        CapturedParam::Lifetime(lifetime) => lifetime::to_ident(lifetime),
        CapturedParam::Ident(ident) => ident.clone(),
        unsupported => panic!(
            "Unsupported captured parameter: {}",
            unsupported.to_token_stream().to_string()
        ),
    };
    return result;
}
