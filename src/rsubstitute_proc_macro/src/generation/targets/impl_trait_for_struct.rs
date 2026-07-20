mod as_trait_control_impl;

use crate::common::models::*;
use crate::generation::mock_controls::models::ControlType;
use crate::generation::mock_controls::*;
use crate::generation::mock_struct::models::*;
use crate::generation::targets::common::*;
use crate::generation::targets::models::*;
use crate::generation::targets::*;
use crate::generation::*;
use crate::preparation::r#struct::*;
use crate::syntax::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, mut item_impl: ItemImpl) -> MockMod {
    let source_span = item_impl.span();
    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        panic!("When mocking trait implementation `item_impl` must have trait field.")
    };
    let impl_trait_for_struct_syntax =
        impl_trait_for_struct_syntax::prepare(impl_trait_for_struct_syntax::Params {
            attributes: item_impl.attrs.clone(),
            merged_generics: item_impl.generics.clone(),
            target_type: item_impl.self_ty.clone(),
            trait_path: trait_path.clone(),
            impl_items: item_impl.items.clone(),
        });
    let impl_trait_for_struct_info =
        impl_trait_for_struct_info::generate(ctx, impl_trait_for_struct_syntax);
    let mock_struct_path = path::from_base_path_with_ident(
        &impl_trait_for_struct_info.target_path,
        format_ident!(
            "{}Mock",
            path::last_ident(&impl_trait_for_struct_info.target_path)
        ),
    );

    let maybe_associated_controls =
        (!impl_trait_for_struct_info.associated_fns.is_empty()).then(|| {
            let trait_setup_struct = setup::generate(
                ctx,
                source_span,
                setup::Params {
                    ident: path::last_ident(&impl_trait_for_struct_info.target_path),
                    generics: impl_trait_for_struct_info.merged_generics.clone(),
                    mock_struct_path: &mock_struct_path,
                    fn_infos: &impl_trait_for_struct_info.associated_fns,
                },
            );
            let trait_received_struct = received::generate(
                ctx,
                source_span,
                received::Params {
                    ident: path::last_ident(&impl_trait_for_struct_info.target_path),
                    generics: impl_trait_for_struct_info.merged_generics.clone(),
                    mock_struct_path: &mock_struct_path,
                    fn_infos: &impl_trait_for_struct_info.associated_fns,
                },
            );
            let setup_struct_impl = as_trait_control_impl::generate(
                source_span,
                as_trait_control_impl::Params {
                    struct_path: &impl_trait_for_struct_info.target_path,
                    struct_generics: impl_trait_for_struct_info.target_simple_generics.clone(),
                    trait_ident: &impl_trait_for_struct_info.trait_ident,
                    trait_generics: impl_trait_for_struct_info.trait_simple_generics.clone(),
                    maybe_common_where_clause: impl_trait_for_struct_info
                        .merged_generics
                        .where_clause
                        .clone(),
                    control_type: ControlType::Setup,
                    is_static: false,
                },
            );
            let received_struct_impl = as_trait_control_impl::generate(
                source_span,
                as_trait_control_impl::Params {
                    struct_path: &impl_trait_for_struct_info.target_path,
                    struct_generics: impl_trait_for_struct_info.target_simple_generics.clone(),
                    trait_ident: &impl_trait_for_struct_info.trait_ident,
                    trait_generics: impl_trait_for_struct_info.trait_simple_generics.clone(),
                    maybe_common_where_clause: impl_trait_for_struct_info
                        .merged_generics
                        .where_clause
                        .clone(),
                    control_type: ControlType::Received,
                    is_static: false,
                },
            );
            let associated_controls = TraitAssociatedControls {
                trait_setup_struct,
                trait_received_struct,
                setup_struct_impl,
                received_struct_impl,
            };
            return associated_controls;
        });
    let maybe_static_controls = (!impl_trait_for_struct_info.static_fns.is_empty()).then(|| {
        let trait_static_setup_struct = static_setup::generate(
            ctx,
            source_span,
            static_setup::Params {
                ident: path::last_ident(&impl_trait_for_struct_info.target_path),
                generics: impl_trait_for_struct_info.merged_generics.clone(),
                maybe_argument_types: None,
                mock_struct_path: &mock_struct_path,
                fn_infos: &impl_trait_for_struct_info.static_fns,
                for_static_fn: false,
            },
        );
        let trait_static_received_struct = static_received::generate(
            ctx,
            source_span,
            static_received::Params {
                ident: path::last_ident(&impl_trait_for_struct_info.target_path),
                generics: impl_trait_for_struct_info.merged_generics.clone(),
                maybe_argument_types: None,
                mock_struct_path: &mock_struct_path,
                fn_infos: &impl_trait_for_struct_info.static_fns,
                for_static_fn: false,
            },
        );
        let static_setup_struct_impl = as_trait_control_impl::generate(
            source_span,
            as_trait_control_impl::Params {
                struct_path: &impl_trait_for_struct_info.target_path,
                struct_generics: impl_trait_for_struct_info.target_simple_generics.clone(),
                trait_ident: &impl_trait_for_struct_info.trait_ident,
                trait_generics: impl_trait_for_struct_info.trait_simple_generics.clone(),
                maybe_common_where_clause: impl_trait_for_struct_info
                    .merged_generics
                    .where_clause
                    .clone(),
                control_type: ControlType::Setup,
                is_static: true,
            },
        );
        let static_received_struct_impl = as_trait_control_impl::generate(
            source_span,
            as_trait_control_impl::Params {
                struct_path: &impl_trait_for_struct_info.target_path,
                struct_generics: impl_trait_for_struct_info.target_simple_generics.clone(),
                trait_ident: &impl_trait_for_struct_info.trait_ident,
                trait_generics: impl_trait_for_struct_info.trait_simple_generics.clone(),
                maybe_common_where_clause: impl_trait_for_struct_info
                    .merged_generics
                    .where_clause
                    .clone(),
                control_type: ControlType::Received,
                is_static: true,
            },
        );
        let static_controls = TraitStaticControls {
            trait_static_setup_struct,
            trait_static_received_struct,
            static_setup_struct_impl,
            static_received_struct_impl,
        };
        return static_controls;
    });
    source_static_fn_block::replace(source_span, mock_struct_path.clone(), &mut item_impl);
    let mock_struct_impl = mock_struct_impl::generate(
        ctx,
        source_span,
        mock_struct_impl::Params {
            mock_struct_path: mock_struct_path.clone(),
            associated_fns: &impl_trait_for_struct_info.associated_fns,
            static_fns: &impl_trait_for_struct_info.static_fns,
            generics: impl_trait_for_struct_info.merged_generics,
        },
    );

    let mock_mod_usages = mock_mod_usages::new(source_span);
    let items = [
        Item::Use(mock_mod_usages.use_rsubstitute_for_generated),
        Item::Use(mock_mod_usages.use_super),
        Item::Impl(item_impl),
        Item::Impl(mock_struct_impl),
    ]
    .into_iter()
    .chain(maybe_associated_controls.into_iter().flat_map(|x| {
        [
            Item::Struct(x.trait_setup_struct.item_struct),
            Item::Impl(x.trait_setup_struct.item_impl),
            Item::Struct(x.trait_received_struct.item_struct),
            Item::Impl(x.trait_received_struct.clone_impl),
            Item::Impl(x.trait_received_struct.item_impl),
            Item::Impl(x.setup_struct_impl),
            Item::Impl(x.received_struct_impl),
        ]
    }))
    .chain(maybe_static_controls.into_iter().flat_map(|x| {
        [
            Item::Struct(x.trait_static_setup_struct.item_struct),
            Item::Impl(x.trait_static_setup_struct.item_impl),
            Item::Struct(x.trait_static_received_struct.item_struct),
            Item::Impl(x.trait_static_received_struct.clone_impl),
            Item::Impl(x.trait_static_received_struct.item_impl),
            Item::Impl(x.static_setup_struct_impl),
            Item::Impl(x.static_received_struct_impl),
        ]
    }))
    .collect();
    let call_site = proc_macro::Span::call_site();
    let line = call_site.line();
    let column = call_site.column();
    let mod_ident = format_ident!(
        "__rsubstitute_generated_{}_{}_{}",
        path::last_ident(&impl_trait_for_struct_info.target_path),
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
