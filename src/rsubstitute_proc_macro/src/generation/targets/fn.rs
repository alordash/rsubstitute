mod base_fn;
mod mocked_fn;

use crate::generation::*;
use crate::preparation::models::Context;
use crate::preparation::r#fn::fn_syntax;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: Context, item_fn: ItemFn) -> ItemMod {
    let source_span = item_fn.span();
    let fn_syntax = fn_syntax::prepare(fn_syntax::Params {
        attributes: item_fn.attrs,
        visibility: item_fn.vis,
        signature: item_fn.sig,
        is_default: false,
        maybe_base_impl: Some(item_fn.block),
        maybe_owner: None,
    });
    let mut fn_info = fn_info::generate(fn_syntax);

    let mock_struct = mock_struct::generate_for_static_fn(source_span, &fn_info.syntax);

    let base_impl = fn_info
        .syntax
        .maybe_base_impl
        .take()
        .expect("Static `fn`s should always have base implementation.");
    let base_fn = base_fn::generate(source_span, &fn_info, mock_struct.path.clone(), base_impl);

    let mod_ident = fn_info.syntax.source_signature.ident.clone();
    let mocked_fn = mocked_fn::generate(
        source_span,
        fn_info,
        mock_struct.path,
        base_fn.sig.ident.clone(),
    );
    let items = vec![Item::Fn(mocked_fn), Item::Fn(base_fn)];

    let result = ItemMod {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](source_span)),
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    return result;
}
