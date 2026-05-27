use crate::syntax::bound_lifetimes;
use syn::*;

pub(crate) fn to_ident(type_bare_fn: &TypeBareFn) -> Ident {
    let lifetimes = type_bare_fn
        .lifetimes
        .as_ref()
        .map(bound_lifetimes::to_ident);

    todo!()
}
