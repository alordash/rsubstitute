use crate::common::models::*;
use crate::generation::mock_controls::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use crate::generation::targets::models::*;
use crate::generation::*;
use crate::preparation::r#trait::*;
use crate::syntax::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, item_trait: ItemTrait) -> MockMod {
    let source_span = item_trait.span();
    let trait_syntax = trait_syntax::prepare(trait_syntax::Params {
        attributes: item_trait.attrs.clone(),
        unsafety: item_trait.unsafety.clone(),
        visibility: item_trait.vis.clone(),
        ident: item_trait.ident.clone(),
        generics: item_trait.generics.clone(),
        items: item_trait.items.clone(),
    });
    let trait_info = trait_info::generate(ctx, trait_syntax);

    let trait_mock_struct_ident = format_ident!("{}Mock", trait_info.ident);
    let trait_mock_struct_path = path::from_ident_with_generics(
        trait_mock_struct_ident.clone(),
        &trait_info.merged_generics,
    );
    let maybe_associated_controls = (!trait_info.associated_fns.is_empty()).then(|| {
        let setup_struct = setup::generate(
            ctx,
            source_span,
            setup::Params {
                ident: trait_info.ident.clone(),
                generics: trait_info.merged_generics.clone(),
                mock_struct_path: trait_mock_struct_path.clone(),
                fn_infos: &trait_info.associated_fns,
            },
        );
        let received_struct = received::generate(
            ctx,
            source_span,
            received::Params {
                ident: trait_info.ident.clone(),
                generics: trait_info.merged_generics.clone(),
                mock_struct_path: trait_mock_struct_path.clone(),
                fn_infos: &trait_info.associated_fns,
            },
        );
        let associated_controls = AssociatedControls {
            setup_struct,
            received_struct,
        };
        return associated_controls;
    });
    let maybe_static_controls = (!trait_info.static_fns.is_empty()).then(|| {
        let static_setup_struct = static_setup::generate(
            ctx,
            source_span,
            static_setup::Params {
                ident: trait_info.ident.clone(),
                generics: trait_info.merged_generics.clone(),
                maybe_argument_types: None,
                mock_struct_path: &trait_mock_struct_path,
                fn_infos: &trait_info.static_fns,
            },
        );
        let static_received_struct = static_received::generate(
            ctx,
            source_span,
            static_received::Params {
                ident: trait_info.ident.clone(),
                generics: trait_info.merged_generics.clone(),
                maybe_argument_types: None,
                mock_struct_path: &trait_mock_struct_path,
                fn_infos: &trait_info.static_fns,
                for_static_fn: false,
            },
        );
        let static_controls = StaticControls {
            static_setup_struct,
            static_received_struct,
        };
        return static_controls;
    });
    let trait_mock_struct = trait_mock_struct::generate(
        ctx,
        source_span,
        trait_mock_struct::Params {
            mock_struct_ident: trait_mock_struct_ident,
            trait_info: &trait_info,
            maybe_associated_controls,
            maybe_static_controls,
        },
    );

    let mod_visibility = item_trait.vis.clone();
    let items = [Item::Trait(item_trait)].into_iter().collect();

    let usage = todo!();
    let item_mod = ItemMod {
        attrs: vec![attributes::allow_non_camel_case_types(source_span)],
        vis: mod_visibility,
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: todo!(),
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod { usage, item_mod };
    return result;
}
