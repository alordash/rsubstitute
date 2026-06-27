mod base_fn;
mod mocked_fn;

use crate::generation::fn_info::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate_module(source_span: Span, fn_info: FnInfo) -> ItemMod {
    let attrs = vec![];
    let ident = fn_info.syntax.source_signature.ident.clone();

    let mock_path = todo!();

    let base_fn = base_fn::generate(source_span, &fn_info);

    let mocked_fn = mocked_fn::generate(source_span, fn_info, mock_path, base_fn.sig.ident.clone());
    let items = vec![Item::Fn(mocked_fn), Item::Fn(base_fn)];

    let result = ItemMod {
        attrs,
        vis: Visibility::Public(Token![pub](source_span)),
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    return result;
}
