mod base_fn;
mod mocked_fn;

use crate::common::models::*;
use crate::generation::mock_controls::*;
use crate::generation::targets::mock_mod_usages;
use crate::generation::targets::models::*;
use crate::generation::*;
use crate::preparation::r#fn::fn_syntax;
use crate::preparation::r#fn::models::IArgumentTypesCloner;
use crate::syntax::attributes;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, item_fn: ItemFn) -> MockMod {
    let source_span = item_fn.span();
    let fn_syntax = fn_syntax::prepare(fn_syntax::Params {
        attributes: item_fn.attrs,
        visibility: item_fn.vis,
        signature: item_fn.sig,
        is_default: false,
        maybe_base_impl: Some(item_fn.block),
        maybe_owner: None,
    });
    let mut fn_info = fn_info::generate(ctx, fn_syntax);

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
    let target_argument_types: Vec<_> = fn_info.syntax.arguments.iter_generics_style_types().collect();
    let fn_infos = [fn_info];
    let static_setup_struct = static_setup::generate(static_setup::Params {
        ctx,
        source_span,
        target_ident: target_ident.clone(),
        target_generics: target_generics.clone(),
        maybe_target_argument_types: Some(target_argument_types.clone()),
        mock_path: &mock_struct.path,
        fn_infos: &fn_infos,
    });
    let static_received_struct = static_received::generate(static_received::Params {
        ctx,
        source_span,
        target_ident,
        target_generics,
        maybe_target_argument_types: Some(target_argument_types),
        mock_path: &mock_struct.path,
        fn_infos: &fn_infos,
        static_no_other_calls: true,
    });
    let [fn_info] = fn_infos;
    let fn_static_setup = fn_static_setup::generate(
        ctx,
        source_span,
        mock_struct.path.clone(),
        static_setup_struct.path.clone(),
        &fn_info,
    );
    let fn_static_received =
        fn_static_received::generate(source_span, static_received_struct.path.clone(), &fn_info);

    let mod_ident = fn_info.syntax.source_signature.ident.clone();
    let mocked_fn = mocked_fn::generate(
        ctx,
        source_span,
        &fn_info,
        mock_struct.path,
        maybe_base_fn.as_ref().map(|x| x.sig.ident.clone()),
    );
    let usage_ident = mocked_fn.sig.ident.clone();
    let fn_items = if let Some(base_fn) = maybe_base_fn {
        vec![Item::Fn(mocked_fn), Item::Fn(base_fn)]
    } else {
        vec![Item::Fn(mocked_fn)]
    };

    let mock_mod_usages = mock_mod_usages::new(source_span);

    let items = [
        Item::Use(mock_mod_usages.use_rsubstitute_for_generated),
        Item::Use(mock_mod_usages.use_super),
    ]
    .into_iter()
    .chain(fn_items)
    .chain([
        Item::Fn(fn_static_setup),
        Item::Fn(fn_static_received),
        Item::Struct(fn_info.call_struct.item_struct),
        Item::Impl(fn_info.call_struct.generics_info_provider_impl),
        Item::Impl(fn_info.call_struct.call_impl),
    ])
    .chain(
        fn_info
            .call_struct
            .maybe_clone_impl
            .map(Item::Impl)
            .into_iter(),
    )
    .chain([
        Item::Struct(fn_info.args_checker_struct.item_struct),
        Item::Impl(fn_info.args_checker_struct.generics_info_provider_impl),
        Item::Impl(fn_info.args_checker_struct.args_checker_impl),
        Item::Struct(mock_struct.item_struct),
        Item::Struct(static_setup_struct.item_struct),
        Item::Impl(static_setup_struct.item_impl),
        Item::Struct(static_received_struct.item_struct),
        Item::Impl(static_received_struct.item_impl),
    ])
    .collect();

    let usage = ItemUse {
        attrs: Vec::new(),
        vis: fn_info.syntax.visibility.clone(),
        use_token: Token![use](source_span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: mod_ident.clone(),
            colon2_token: Token![::](source_span),
            tree: Box::new(UseTree::Name(UseName { ident: usage_ident })),
        }),
        semi_token: Token![;](source_span),
    };
    let item_mod = ItemMod {
        attrs: vec![attributes::allow_non_camel_case_types(source_span)],
        vis: Visibility::Public(Token![pub](source_span)),
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod { usage, item_mod };
    return result;
}
