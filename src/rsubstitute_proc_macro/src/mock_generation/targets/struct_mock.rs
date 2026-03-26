use crate::constants;
use crate::mock_generation::fn_info_generation::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::mock_generation::models::*;
use crate::mock_generation::parameters::Target;
use crate::mock_generation::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub(crate) fn handle(ctx: &Ctx, mut struct_mock_syntax: StructMockSyntax) -> TokenStream {
    let source_struct_impls_syntax = generate_source_struct_impls_syntax(&struct_mock_syntax);

    let mock_ident = struct_mock_syntax.r#struct.ident.clone();

    let mock_generics = mock_generics::generate_for_struct(
        &struct_mock_syntax.r#struct.generics,
        &struct_mock_syntax.trait_impls,
    );
    let mock_type = mock_type::generate_for_struct(mock_ident.clone(), mock_generics);
    let mock_struct_trait_infos: Vec<_> = core::mem::take(&mut struct_mock_syntax.trait_impls)
        .into_iter()
        .map(|x| mock_struct_trait_info::generate(ctx, &mock_type, x))
        .collect();
    let struct_fn_decls =
        fn_decl::extract_struct_fns(ctx, &mock_type, &struct_mock_syntax.get_struct_fns());
    let target_ident = struct_mock_syntax.r#struct.ident.clone();
    let struct_fn_infos: Vec<_> = struct_fn_decls
        .into_iter()
        .map(|x| fn_info::generate(ctx, x, &mock_type, Target::TraitOrStruct))
        .collect();
    let all_fn_infos: Vec<_> = struct_fn_infos
        .iter()
        .chain(mock_struct_trait_infos.iter().flat_map(|x| &x.fn_infos))
        .collect();
    let mock_data_struct = mock_data_struct::generate_for_trait(&mock_type, &all_fn_infos, true);
    let mut mock_struct_traits: Vec<_> = mock_struct_trait_infos
        .into_iter()
        .map(|mock_struct_trait_info| {
            mock_struct_trait::generate(&mock_data_struct, mock_struct_trait_info)
        })
        .collect();
    let mock_setup_struct = mock_setup_struct::generate(
        &mock_ident,
        &mock_type,
        &mock_data_struct,
        mock_struct_traits
            .iter()
            .map(|mock_struct_trait| ImplementedTraitConfigurator {
                trait_ident: mock_struct_trait.info.trait_ident_from_path.clone(),
                item_struct: &mock_struct_trait.setup_struct.item_struct,
            })
            .collect(),
    );
    let mock_received_struct = mock_received_struct::generate(
        &mock_ident,
        &mock_type,
        &mock_data_struct,
        mock_struct_traits
            .iter()
            .map(|mock_struct_trait| ImplementedTraitConfigurator {
                trait_ident: mock_struct_trait.info.trait_ident_from_path.clone(),
                item_struct: &mock_struct_trait.received_struct.item_struct,
            })
            .collect(),
    );
    let struct_attrs = struct_mock_syntax.r#struct.attrs.clone();
    let struct_impls_attrs = struct_mock_syntax.get_struct_impls_attrs();
    let inner_data_struct = inner_data_struct::generate(struct_mock_syntax.r#struct);
    let mock_struct = mock_struct::generate(
        struct_attrs,
        &mock_type,
        &mock_setup_struct,
        &mock_received_struct,
        &mock_data_struct,
        Some(&inner_data_struct),
        false,
    );
    let inner_data_deref_impl = inner_data_deref_impl::generate(&mock_struct, &inner_data_struct);
    let mock_trait_impls = mock_struct_traits
        .iter_mut()
        .map(|mock_struct_trait| {
            mock_payload_impl::generate_for_struct_trait(
                mock_struct_trait.info.trait_path.clone(),
                &mock_type,
                &mock_struct_trait.info.fn_infos,
                &mock_struct_trait.info.trait_ident_from_path,
                core::mem::take(&mut mock_struct_trait.info.rest_impl_items),
            )
        })
        .collect();
    let mock_struct_impl =
        mock_payload_impl::generate_for_struct(struct_impls_attrs, &mock_type, &struct_fn_infos);
    let inner_data_param =
        inner_data_param::generate(&inner_data_struct, &struct_mock_syntax.new_fn);
    let inner_data_impl = inner_data_impl::generate(&inner_data_struct, struct_mock_syntax.new_fn);
    let struct_base_fns: Vec<_> = struct_fn_infos
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
    let struct_traits_base_fns: Vec<_> = mock_struct_traits
        .iter()
        .flat_map(|mock_struct_trait| {
            mock_struct_trait
                .info
                .fn_infos
                .iter()
                .flat_map(|trait_fn_info| {
                    trait_fn_info
                        .parent
                        .maybe_base_fn_block
                        .clone()
                        .map(|base_fn_block| {
                            base_fn::generate_struct_trait_fn(
                                &mock_type,
                                &trait_fn_info.parent,
                                &trait_fn_info.call_struct,
                                base_fn_block,
                                &mock_struct_trait.info.trait_ident_from_path,
                            )
                        })
                })
        })
        .map(|base_fn| ImplItem::Fn(base_fn.impl_item_fn))
        .collect();
    let all_base_fns = struct_base_fns
        .into_iter()
        .chain(struct_traits_base_fns)
        .collect();
    let mock_impl = mock_impl::generate(
        &mock_type,
        &mock_struct,
        &mock_data_struct,
        &mock_setup_struct,
        &mock_received_struct,
        mock_struct_traits.iter().collect(),
        Some(inner_data_param),
        all_base_fns,
    );
    let mock_setup_impl =
        mock_setup_impl::generate_for_trait(&mock_type, &mock_setup_struct, &struct_fn_infos);
    let mock_received_impl =
        mock_received_impl::generate_for_trait(&mock_type, &mock_received_struct, &struct_fn_infos);
    ignored_impl::fix(&mock_type, &mut struct_mock_syntax.ignored_impls);
    let generated_mod = module::generate_struct(
        target_ident,
        mock_struct_traits,
        struct_fn_infos,
        mock_data_struct,
        mock_setup_struct,
        mock_received_struct,
        inner_data_struct,
        inner_data_impl,
        mock_struct,
        inner_data_deref_impl,
        mock_trait_impls,
        mock_struct_impl,
        mock_impl,
        mock_setup_impl,
        mock_received_impl,
        struct_mock_syntax.ignored_impls,
    );

    let GeneratedMod {
        item_mod,
        use_generated_mod,
    } = generated_mod;
    let result = quote! {
        #source_struct_impls_syntax

        #use_generated_mod
        #item_mod
    };
    return result.into();
}

fn generate_source_struct_impls_syntax(
    struct_mock_syntax: &StructMockSyntax,
) -> proc_macro2::TokenStream {
    let cfg_not_test_attribute = constants::CFG_NOT_TEST_ATTRIBUTE.clone();
    let struct_syntax_var = &struct_mock_syntax.r#struct;
    let struct_syntax = quote! {
        #cfg_not_test_attribute
        #struct_syntax_var
    };

    let trait_impls_syntaxes: Vec<_> = struct_mock_syntax
        .trait_impls
        .iter()
        .map(|trait_impl| {
            let item_impl = &trait_impl.item_impl;
            quote! {
                #cfg_not_test_attribute
                #item_impl
            }
        })
        .collect();

    let struct_impls_syntaxes: Vec<_> = struct_mock_syntax
        .struct_impls
        .iter()
        .map(|struct_impl| {
            quote! {
                #cfg_not_test_attribute
                #struct_impl
            }
        })
        .collect();

    let result = quote! {
        #struct_syntax

        #(#trait_impls_syntaxes)*

        #(#struct_impls_syntaxes)*
    };
    return result;
}
