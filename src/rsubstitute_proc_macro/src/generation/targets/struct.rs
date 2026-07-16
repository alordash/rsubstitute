mod mockable_trait;
mod mockable_trait_impl;
mod struct_control_struct;

use crate::generation::mock_controls::models::ControlType;
use crate::generation::mock_struct::struct_mock_struct;
use crate::generation::targets::common::*;
use crate::generation::targets::models::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(mut item_struct: ItemStruct) -> MockMod {
    let source_span = item_struct.span();
    let mockable_trait = mockable_trait::generate(source_span);
    let struct_setup_struct = struct_control_struct::generate(
        source_span,
        struct_control_struct::Params {
            struct_ident: &item_struct.ident,
            generics: item_struct.generics.clone(),
            control_type: ControlType::Setup,
            is_static: false,
        },
    );
    let struct_received_struct = struct_control_struct::generate(
        source_span,
        struct_control_struct::Params {
            struct_ident: &item_struct.ident,
            generics: item_struct.generics.clone(),
            control_type: ControlType::Received,
            is_static: false,
        },
    );
    let struct_static_setup_struct = struct_control_struct::generate(
        source_span,
        struct_control_struct::Params {
            struct_ident: &item_struct.ident,
            generics: item_struct.generics.clone(),
            control_type: ControlType::Setup,
            is_static: true,
        },
    );
    let struct_static_received_struct = struct_control_struct::generate(
        source_span,
        struct_control_struct::Params {
            struct_ident: &item_struct.ident,
            generics: item_struct.generics.clone(),
            control_type: ControlType::Received,
            is_static: true,
        },
    );
    let struct_mock_ident = format_ident!("{}Mock", item_struct.ident);
    let struct_mock_struct = struct_mock_struct::generate(
        source_span,
        struct_mock_struct::Params {
            struct_ident: item_struct.ident.clone(),
            struct_mock_ident: struct_mock_ident.clone(),
            generics: item_struct.generics.clone(),
            struct_setup_ident: struct_setup_struct.ident.clone(),
            struct_received_ident: struct_received_struct.ident.clone(),
        },
    );
    let mockable_trait_impl = mockable_trait_impl::generate(
        source_span,
        mockable_trait_impl::Params {
            struct_ident: item_struct.ident.clone(),
            generics: item_struct.generics.clone(),
            struct_mock_ident: struct_mock_struct.item_struct.ident.clone(),
            static_setup_struct_ident: struct_static_setup_struct.ident.clone(),
            static_received_struct_ident: struct_static_received_struct.ident.clone(),
        },
    );

    let mod_visibility = item_struct.vis.clone();
    let mod_ident = format_ident!("__rsubstitute_generated_{}Mock", item_struct.ident);
    let usage = mod_usage::new(
        mod_ident.clone(),
        [
            item_struct.ident.clone(),
            struct_mock_struct.item_struct.ident.clone(),
        ],
    );
    item_struct.vis = Visibility::Public(Token![pub](source_span));
    let items = vec![
        Item::Struct(item_struct),
        Item::Trait(mockable_trait),
        Item::Impl(mockable_trait_impl),
        Item::Struct(struct_mock_struct.item_struct),
        Item::Impl(struct_mock_struct.item_impl),
        Item::Impl(struct_mock_struct.deref_impl),
        Item::Impl(struct_mock_struct.deref_mut_impl),
        Item::Struct(struct_setup_struct),
        Item::Struct(struct_received_struct),
        Item::Struct(struct_static_setup_struct),
        Item::Struct(struct_static_received_struct),
    ];
    let item_mod = ItemMod {
        attrs: Vec::new(),
        vis: mod_visibility,
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod { usage, item_mod };
    return result;
}
