use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::GeneratedMod;
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
        item_fn: &ItemFn,
        base_fn: BaseFn,
        fn_info: FnInfo,
        base_caller_struct: BaseCallerStruct,
        base_caller_impl: BaseCallerImpl,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        mock_struct: MockStruct,
        send_sync_impls: SendSyncImpls,
        mock_setup_impl: MockSetupImpl,
        mock_received_impl: MockReceivedImpl,
        static_mock: StaticMock,
        fn_setup: ItemFn,
        fn_received: ItemFn,
    ) -> GeneratedMod;
}

pub struct ModGenerator;

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
        let attrs = vec![constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone()];
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
                    Item::Struct(x.args_checker_struct.item_struct),
                    Item::Impl(x.args_checker_impl.item_impl),
                ]
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
            attrs,
            vis: Visibility::Inherited,
            unsafety: None,
            mod_token: Default::default(),
            ident,
            content: Some((Default::default(), items)),
            semi: None,
        };
        let use_generated_mod = ItemUse {
            attrs: Vec::new(),
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
        item_fn: &ItemFn,
        base_fn: BaseFn,
        fn_info: FnInfo,
        base_caller_struct: BaseCallerStruct,
        base_caller_impl: BaseCallerImpl,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        mock_struct: MockStruct,
        send_sync_impls: SendSyncImpls,
        mock_setup_impl: MockSetupImpl,
        mock_received_impl: MockReceivedImpl,
        static_mock: StaticMock,
        fn_setup: ItemFn,
        fn_received: ItemFn,
    ) -> GeneratedMod {
        let fn_ident = item_fn.sig.ident.clone();
        let attrs = vec![constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone()];
        let usings = [
            constants::USE_SUPER.clone(),
            constants::USE_FOR_GENERATED.clone(),
        ];
        let items = usings
            .into_iter()
            .map(|x| Item::Use(x))
            .chain([
                Item::Fn(base_fn.item_fn),
                Item::Struct(fn_info.call_struct.item_struct),
                Item::Struct(fn_info.args_checker_struct.item_struct),
                Item::Impl(fn_info.args_checker_impl.item_impl),
                Item::Struct(base_caller_struct.item_struct),
                Item::Impl(base_caller_impl.item_impl),
                Item::Struct(mock_data_struct.item_struct),
                Item::Struct(mock_setup_struct.item_struct),
                Item::Struct(mock_received_struct.item_struct),
                Item::Struct(mock_struct.item_struct),
                Item::Impl(send_sync_impls.send_impl),
                Item::Impl(send_sync_impls.sync_impl),
                Item::Impl(mock_setup_impl.item_impl),
                Item::Impl(mock_received_impl.item_impl),
                Item::Static(static_mock.item_static),
                Item::Fn(fn_setup),
                Item::Fn(fn_received),
            ])
            .collect();
        let item_mod = ItemMod {
            attrs,
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
