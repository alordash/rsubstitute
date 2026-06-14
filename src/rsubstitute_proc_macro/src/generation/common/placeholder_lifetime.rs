use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span) -> Lifetime {
    let result = Lifetime::new("_", span);

    return result;
}
