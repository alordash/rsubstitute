use crate::constants;
use crate::mock_generation::fn_info_generation::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::mock_generation::models::*;
use crate::mock_generation::parameters::Target;
use crate::mock_generation::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*;

pub(crate) fn handle(ctx: &Ctx, item_trait: ItemTrait) -> TokenStream {
    let mock_ident = format_ident!(
        "{}{}",
        item_trait.ident,
        constants::MOCK_STRUCT_IDENT_PREFIX
    );
    let associated_generics = associated_generics::extract(&item_trait.ident, &item_trait.items);
    let mock_generics = mock_generics::generate(
        &item_trait.generics,
        Target::Trait,
        Some(&associated_generics),
    );
    let fn_decls = fn_decl::extract(ctx, &mock_generics, &item_trait.items);
    let target_ident = item_trait.ident.clone();
    let mock_type = mock_type::generate(mock_ident.clone(), mock_generics);
    let fn_infos: Vec<_> = fn_decls
        .into_iter()
        .map(|x| fn_info::generate(ctx, x, &mock_type))
        .collect();
    let all_fn_infos: Vec<_> = fn_infos.iter().collect();
    let mock_data_struct = mock_data_struct::generate_for_trait(&mock_type, &all_fn_infos);
    let mock_setup_struct =
        mock_setup_struct::generate(&mock_ident, &mock_type, &mock_data_struct, Vec::new());
    let mock_received_struct =
        mock_received_struct::generate(&mock_ident, &mock_type, &mock_data_struct, Vec::new());
    let mock_struct = mock_struct::generate(
        vec![constants::DERIVE_CLONE_FOR_RSUBSTITUTE_ATTRIBUTE.clone()],
        &mock_type,
        &mock_setup_struct,
        &mock_received_struct,
        &mock_data_struct,
        None,
    );
    let mock_trait_impl = mock_payload_impl::generate(
        target_ident.clone(),
        &mock_type,
        &fn_infos,
        associated_generics,
    );
    let base_fns: Vec<_> = fn_infos
        .iter()
        .filter_map(|fn_info| {
            fn_info
                .parent
                .maybe_base_fn_block
                .clone()
                .map(|base_fn_block| {
                    base_fn::generate(
                        &mock_type,
                        &fn_info.parent,
                        &fn_info.call_struct,
                        base_fn_block,
                    )
                })
        })
        .map(|base_fn| ImplItem::Fn(base_fn.impl_item_fn))
        .collect();
    let mock_impl = mock_impl::generate(
        &mock_type,
        &mock_struct,
        &mock_data_struct,
        &mock_setup_struct,
        &mock_received_struct,
        Vec::new(),
        None,
        base_fns,
    );
    let mock_setup_impl =
        mock_setup_impl::generate_for_trait(&mock_type, &mock_setup_struct, &fn_infos);
    let mock_received_impl =
        mock_received_impl::generate_for_trait(&mock_type, &mock_received_struct, &fn_infos);
    let generated_mod = module::generate_trait(
        target_ident,
        fn_infos,
        mock_data_struct,
        mock_setup_struct,
        mock_received_struct,
        mock_struct,
        mock_trait_impl,
        mock_impl,
        mock_setup_impl,
        mock_received_impl,
    );

    let GeneratedMod {
        item_mod,
        use_generated_mod,
    } = generated_mod;
    let result = quote! {
        #item_trait

        #use_generated_mod
        #item_mod
    };
    return result.into();
}
