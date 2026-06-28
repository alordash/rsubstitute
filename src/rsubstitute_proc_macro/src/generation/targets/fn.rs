mod base_fn;
mod mocked_fn;

use crate::common::models::*;
use crate::generation::mock_controls::*;
use crate::generation::*;
use crate::preparation::r#fn::fn_syntax;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, item_fn: ItemFn) -> ItemMod {
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

    let maybe_base_fn = if ctx.support_base_calling {
        let base_impl = fn_info
            .syntax
            .maybe_base_impl
            .take()
            .expect("Static `fn`s should always have base implementation.");
        Some(base_fn::generate(
            source_span,
            &fn_info,
            mock_struct.path.clone(),
            base_impl,
        ))
    } else {
        None
    };
    let target_ident = fn_info.syntax.fn_ident.clone();
    let target_generics = fn_info.syntax.merged_generics.clone();
    let fn_infos = [fn_info];
    let static_setup_struct = static_setup::generate(static_setup::Params {
        ctx,
        source_span,
        target_ident,
        target_generics,
        mock_path: &mock_struct.path,
        fn_infos: &fn_infos,
    });
    let [fn_info] = fn_infos;

    let mod_ident = fn_info.syntax.source_signature.ident.clone();
    let mocked_fn = mocked_fn::generate(
        source_span,
        &fn_info,
        mock_struct.path,
        maybe_base_fn.as_ref().map(|x| x.sig.ident.clone()),
    );
    let fn_items = if let Some(base_fn) = maybe_base_fn {
        vec![Item::Fn(mocked_fn), Item::Fn(base_fn)]
    } else {
        vec![Item::Fn(mocked_fn)]
    };

    let items = fn_items
        .into_iter()
        .chain([
            Item::Struct(fn_info.call_struct.item_struct),
            Item::Impl(fn_info.call_struct.generics_info_provider_impl),
            Item::Impl(fn_info.call_struct.call_impl),
            Item::Struct(fn_info.args_checker_struct.item_struct),
            Item::Impl(fn_info.args_checker_struct.generics_info_provider_impl),
            Item::Impl(fn_info.args_checker_struct.args_checker_impl),
            Item::Struct(mock_struct.item_struct),
            Item::Struct(static_setup_struct.item_struct),
            Item::Impl(static_setup_struct.item_impl),
        ])
        .collect();

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
