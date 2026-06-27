use crate::generation::fn_info::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(source_span: Span, fn_info: &FnInfo) -> ItemFn {
    let result = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        sig: todo!(),
        block: todo!(),
    };
    return result;
}
