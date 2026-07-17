mod mock_impl;

use crate::common::models::*;
use crate::generation::mock_controls::*;
use crate::generation::targets::common::*;
use crate::generation::targets::models::*;
use crate::generation::targets::*;
use crate::generation::*;
use crate::preparation::r#struct::*;
use crate::syntax::{attributes, path};
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, item_impl: ItemImpl) -> MockMod {
    let source_span = item_impl.span();
    let impl_struct_syntax = impl_struct_syntax::prepare(impl_struct_syntax::Params {
        attributes: item_impl.attrs.clone(),
        generics: item_impl.generics.clone(),
        target_type: item_impl.self_ty.clone(),
        impl_items: item_impl.items.clone(),
    });
    let impl_struct_info = impl_struct_info::generate(ctx, impl_struct_syntax);
    let mock_struct_path = path::from_ident_with_generics(
        format_ident!("{}Mock", impl_struct_info.target_ident),
        &impl_struct_info.generics,
    );
    let mock_impl = mock_impl::generate(
        ctx,
        source_span,
        mock_struct_path.clone(),
        &impl_struct_info,
    );
    let maybe_associated_controls_impls =
        (!impl_struct_info.associated_fns.is_empty()).then(|| {
            let setup_impl = setup_impl::generate(
                ctx,
                source_span,
                setup_impl::Params {
                    setup_struct_path: path::from_ident_with_generics(
                        format_ident!("{}Setup", impl_struct_info.target_ident),
                        &impl_struct_info.generics,
                    ),
                    generics: impl_struct_info.generics.clone(),
                    mock_struct_path: &mock_struct_path,
                    fn_infos: &impl_struct_info.associated_fns,
                    for_static_fn: false,
                    is_static: false,
                },
            );
            let received_impl = received_impl::generate(
                ctx,
                source_span,
                received_impl::Params {
                    received_struct_path: path::from_ident_with_generics(
                        format_ident!("{}Received", impl_struct_info.target_ident),
                        &impl_struct_info.generics,
                    ),
                    generics: impl_struct_info.generics.clone(),
                    mock_struct_path: &mock_struct_path,
                    fn_infos: &impl_struct_info.associated_fns,
                    for_static_fn: false,
                    is_static: false,
                },
            );
            (setup_impl, received_impl)
        });
    let maybe_static_controls_impls = (!impl_struct_info.static_fns.is_empty()).then(|| {
        let static_setup_impl = setup_impl::generate(
            ctx,
            source_span,
            setup_impl::Params {
                setup_struct_path: path::from_ident_with_generics(
                    format_ident!("{}StaticSetup", impl_struct_info.target_ident),
                    &impl_struct_info.generics,
                ),
                generics: impl_struct_info.generics.clone(),
                mock_struct_path: &mock_struct_path,
                fn_infos: &impl_struct_info.associated_fns,
                for_static_fn: false,
                is_static: true,
            },
        );
        let static_received_impl = received_impl::generate(
            ctx,
            source_span,
            received_impl::Params {
                received_struct_path: path::from_ident_with_generics(
                    format_ident!("{}StaticReceived", impl_struct_info.target_ident),
                    &impl_struct_info.generics,
                ),
                generics: impl_struct_info.generics.clone(),
                mock_struct_path: &mock_struct_path,
                fn_infos: &impl_struct_info.associated_fns,
                for_static_fn: false,
                is_static: true,
            },
        );
        (static_setup_impl, static_received_impl)
    });

    let use_struct_mod = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Token![use](source_span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: Ident::new("super", source_span),
            colon2_token: Token![::](source_span),
            tree: Box::new(UseTree::Path(UsePath {
                ident: format_ident!("__rsubstitute_generated_{}Mock", impl_struct_info.target_ident),
                colon2_token: Token![::](source_span),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Token![*](source_span),
                })),
            })),
        }),
        semi_token: Token![;](source_span),
    };
    let mock_mod_usages = mock_mod_usages::new(source_span);
    let items = [
        Item::Use(mock_mod_usages.use_rsubstitute_for_generated),
        Item::Use(mock_mod_usages.use_super),
        Item::Use(use_struct_mod),
        Item::Impl(item_impl),
    ]
    .into_iter()
    .chain(impl_struct_info.associated_fns.into_iter().flat_map(|x| {
        let call_struct = x.value.call_struct;
        let args_checker = x.value.args_checker_struct;
        [
            Item::Struct(call_struct.item_struct),
            Item::Impl(call_struct.generics_info_provider_impl),
            Item::Impl(call_struct.call_impl),
        ]
        .into_iter()
        .chain(call_struct.maybe_clone_impl.map(Item::Impl).into_iter())
        .chain([
            Item::Struct(args_checker.item_struct),
            Item::Impl(args_checker.generics_info_provider_impl),
            Item::Impl(args_checker.args_checker_impl),
        ])
    }))
    .chain(impl_struct_info.static_fns.into_iter().flat_map(|x| {
        let call_struct = x.value.call_struct;
        let args_checker = x.value.args_checker_struct;
        [
            Item::Struct(call_struct.item_struct),
            Item::Impl(call_struct.generics_info_provider_impl),
            Item::Impl(call_struct.call_impl),
        ]
        .into_iter()
        .chain(call_struct.maybe_clone_impl.map(Item::Impl).into_iter())
        .chain([
            Item::Struct(args_checker.item_struct),
            Item::Impl(args_checker.generics_info_provider_impl),
            Item::Impl(args_checker.args_checker_impl),
        ])
    }))
    .chain(core::iter::once(Item::Impl(mock_impl)))
    .chain(
        maybe_associated_controls_impls
            .into_iter()
            .flat_map(|x| [Item::Impl(x.0), Item::Impl(x.1)]),
    )
    .chain(
        maybe_static_controls_impls
            .into_iter()
            .flat_map(|x| [Item::Impl(x.0), Item::Impl(x.1)]),
    )
    .collect();
    let call_site = proc_macro::Span::call_site();
    let line = call_site.line();
    let column = call_site.column();
    let mod_ident = format_ident!(
        "__rsubstitute_generated_{}_{}_{}",
        impl_struct_info.target_ident,
        line,
        column
    );
    let usage = mod_usage::new_all(mod_ident.clone());
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
