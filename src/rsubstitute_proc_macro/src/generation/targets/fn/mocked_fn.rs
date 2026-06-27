use crate::generation::r#fn::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(source_span: Span, fn_info: FnInfo) -> ItemFn {
    let block = todo!();

    let result = ItemFn {
        attrs: fn_info.syntax.attributes,
        vis: Visibility::Public(Token![pub](source_span)),
        sig: *fn_info.syntax.source_signature,
        block,
    };
    return result;
}
