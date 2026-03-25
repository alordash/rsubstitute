use crate::constants;
use crate::mock_generation::fn_decl;
use crate::mock_generation::fn_info_generation::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::mock_generation::models::Ctx;
use crate::mock_generation::parameters::Target;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*;

pub(crate) fn handle(ctx: &Ctx, item_fn: ItemFn) -> TokenStream {
    let mock_ident = format_ident!(
        "{}{}",
        item_fn.sig.ident,
        constants::MOCK_STRUCT_IDENT_PREFIX
    );
    let mock_generics = mock_generics::generate(&item_fn.sig.generics, Target::Static, None);
    let mock_type = mock_type::generate(mock_ident.clone(), mock_generics);
    let fn_decl = fn_decl::extract_fn(ctx, &mock_type, &item_fn);
    let fn_ident = item_fn.sig.ident.clone();
    let fn_info = fn_info::generate(ctx, fn_decl, &mock_type, Target::Static);
    let fn_infos = [fn_info];
    let all_fn_infos: Vec<_> = fn_infos.iter().collect();
    let mock_data_struct = mock_data_struct::generate_for_static(&mock_type, &all_fn_infos);
    let mock_setup_struct =
        mock_setup_struct::generate(&mock_ident, &mock_type, &mock_data_struct, Vec::new());
    let mock_received_struct =
        mock_received_struct::generate(&mock_ident, &mock_type, &mock_data_struct, Vec::new());
    let mock_struct = mock_struct::generate_for_static(
        &mock_type,
        &mock_setup_struct,
        &mock_received_struct,
        &mock_data_struct,
    );
    let mock_struct_default_impl = mock_struct_default_impl::generate(
        &mock_struct,
        &mock_data_struct,
        &mock_setup_struct,
        &mock_received_struct,
        &mock_type,
    );
    let [fn_info] = fn_infos;
    let mock_setup_impl =
        mock_setup_impl::generate_for_static(&mock_type, &mock_setup_struct, &fn_info);
    let mock_received_impl =
        mock_received_impl::generate_for_static(&mock_type, &mock_received_struct, &fn_info);
    let fn_get_mock = fn_get_mock::generate(&mock_type);
    let fn_setup = fn_setup::generate(&fn_info, &mock_setup_struct, &mock_type);
    let fn_received = fn_received::generate(&fn_info, &mock_received_struct, &mock_type);
    let static_fn = static_fn::generate(&fn_info, &mock_type);
    let maybe_static_base_fn = fn_info
        .parent
        .maybe_base_fn_block
        .clone()
        .map(|base_fn_block| {
            base_fn::generate_static(
                &mock_type,
                &fn_info.parent,
                &fn_info.call_struct,
                base_fn_block,
            )
        });

    let generated_mod = module::generate_fn(
        fn_ident,
        fn_info,
        mock_data_struct,
        mock_setup_struct,
        mock_received_struct,
        mock_struct,
        mock_struct_default_impl,
        mock_setup_impl,
        mock_received_impl,
        fn_get_mock,
        fn_setup,
        fn_received,
        static_fn,
        maybe_static_base_fn,
    );
    let GeneratedMod {
        item_mod,
        use_generated_mod,
    } = generated_mod;

    let cfg_not_test_attribute = constants::CFG_NOT_TEST_ATTRIBUTE.clone();
    let result = quote! {
        #cfg_not_test_attribute
        #item_fn

        #use_generated_mod
        #item_mod
    };
    return result.into();
}
