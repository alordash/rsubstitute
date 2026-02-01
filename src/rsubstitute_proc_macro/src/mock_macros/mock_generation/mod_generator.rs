use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use syn::*;

pub trait IModGenerator {
    fn generate_trait(
        &self,
        trait_ident: Ident,
        fn_infos: Vec<FnInfo>,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        mock_struct: MockStruct,
        mock_trait_impl: MockTraitImpl,
        mock_impl: MockImpl,
        mock_setup_impl: MockSetupImpl,
        mock_received_impl: MockReceivedImpl,
    ) -> GeneratedMod;

    fn generate_fn(
        &self,
        fn_ident: Ident,
        fn_info: FnInfo,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        mock_struct: MockStruct,
        send_sync_impls: SendSyncImpls,
        mock_struct_default_impl: MockStructDefaultImpl,
        mock_setup_impl: MockSetupImpl,
        mock_received_impl: MockReceivedImpl,
        fn_setup: ItemFn,
        fn_received: ItemFn,
        static_fn: StaticFn,
    ) -> GeneratedMod;
}

pub(crate) struct ModGenerator;

impl IModGenerator for ModGenerator {
    fn generate_trait(
        &self,
        trait_ident: Ident,
        fn_infos: Vec<FnInfo>,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        mock_struct: MockStruct,
        mock_trait_impl: MockTraitImpl,
        mock_impl: MockImpl,
        mock_setup_impl: MockSetupImpl,
        mock_received_impl: MockReceivedImpl,
    ) -> GeneratedMod {
        let usings = [
            constants::USE_SUPER.clone(),
            constants::USE_FOR_GENERATED.clone(),
        ];
        let ident = format_ident!("{}_{}", Self::GENERATED_MOD_IDENT.clone(), trait_ident);
        let items = usings
            .into_iter()
            .map(|x| Item::Use(x))
            .chain(fn_infos.into_iter().flat_map(|x| {
                [
                    Item::Struct(x.call_struct.item_struct),
                    Item::Impl(x.call_arg_infos_provider_impl.item_impl),
                    Item::Struct(x.args_checker_struct.item_struct),
                    Item::Impl(x.args_checker_impl.item_impl),
                ]
                .into_iter()
                .chain(
                    x.maybe_base_caller_impl
                        .map(|base_caller_impl| Item::Impl(base_caller_impl.item_impl))
                        .into_iter(),
                )
            }))
            .chain([
                Item::Struct(mock_data_struct.item_struct),
                Item::Struct(mock_setup_struct.item_struct),
                Item::Struct(mock_received_struct.item_struct),
                Item::Struct(mock_struct.item_struct),
                Item::Impl(mock_trait_impl.item_impl),
                Item::Impl(mock_impl.item_impl),
                Item::Impl(mock_setup_impl.item_impl),
                Item::Impl(mock_received_impl.item_impl),
            ])
            .collect();
        let item_mod = ItemMod {
            attrs: vec![
                constants::CFG_TEST_ATTRIBUTE.clone(),
                constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone(),
            ],
            vis: Visibility::Inherited,
            unsafety: None,
            mod_token: Default::default(),
            ident,
            content: Some((Default::default(), items)),
            semi: None,
        };
        let use_generated_mod = ItemUse {
            attrs: vec![constants::CFG_TEST_ATTRIBUTE.clone()],
            vis: Visibility::Public(Default::default()),
            use_token: Default::default(),
            leading_colon: None,
            tree: UseTree::Path(UsePath {
                ident: item_mod.ident.clone(),
                colon2_token: Default::default(),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Default::default(),
                })),
            }),
            semi_token: Default::default(),
        };
        let generated_mod = GeneratedMod {
            item_mod,
            use_generated_mod,
        };
        return generated_mod;
    }

    fn generate_fn(
        &self,
        fn_ident: Ident,
        fn_info: FnInfo,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        mock_struct: MockStruct,
        send_sync_impls: SendSyncImpls,
        mock_struct_default_impl: MockStructDefaultImpl,
        mock_setup_impl: MockSetupImpl,
        mock_received_impl: MockReceivedImpl,
        fn_setup: ItemFn,
        fn_received: ItemFn,
        static_fn: StaticFn,
    ) -> GeneratedMod {
        let usings = [
            constants::USE_SUPER.clone(),
            constants::USE_FOR_GENERATED.clone(),
        ];
        let items = usings
            .into_iter()
            .map(|x| Item::Use(x))
            .chain([
                Item::Struct(fn_info.call_struct.item_struct),
                Item::Impl(fn_info.call_arg_infos_provider_impl.item_impl),
                Item::Struct(fn_info.args_checker_struct.item_struct),
                Item::Impl(fn_info.args_checker_impl.item_impl),
            ])
            .chain(
                fn_info
                    .maybe_base_caller_impl
                    .map(|base_caller_impl| Item::Impl(base_caller_impl.item_impl))
                    .into_iter(),
            )
            .chain([
                Item::Struct(mock_data_struct.item_struct),
                Item::Struct(mock_setup_struct.item_struct),
                Item::Struct(mock_received_struct.item_struct),
                Item::Struct(mock_struct.item_struct),
                Item::Impl(send_sync_impls.send_impl),
                Item::Impl(send_sync_impls.sync_impl),
                Item::Impl(mock_struct_default_impl.item_impl),
                Item::Impl(mock_setup_impl.item_impl),
                Item::Impl(mock_received_impl.item_impl),
                Item::Fn(fn_setup),
                Item::Fn(fn_received),
                Item::Fn(static_fn.item_fn),
            ])
            .collect();
        let item_mod = ItemMod {
            attrs: vec![
                constants::CFG_TEST_ATTRIBUTE.clone(),
                constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone(),
            ],
            vis: Visibility::Inherited,
            unsafety: None,
            mod_token: Default::default(),
            ident: fn_ident.clone(),
            content: Some((Default::default(), items)),
            semi: None,
        };
        let use_generated_mod = ItemUse {
            attrs: vec![constants::CFG_TEST_ATTRIBUTE.clone()],
            vis: Visibility::Inherited,
            use_token: Default::default(),
            leading_colon: None,
            tree: UseTree::Path(UsePath {
                ident: fn_ident.clone(),
                colon2_token: Default::default(),
                tree: Box::new(UseTree::Name(UseName { ident: fn_ident })),
            }),
            semi_token: Default::default(),
        };
        let generated_mod = GeneratedMod {
            item_mod,
            use_generated_mod,
        };
        return generated_mod;
    }
}

impl ModGenerator {
    const GENERATED_MOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("__rsubstitute_generated"));
}
