use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
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

    fn generate_struct(
        &self,
        struct_ident: Ident,
        mock_struct_traits: Vec<MockStructTrait>,
        struct_fn_infos: Vec<FnInfo>,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        inner_data_struct: InnerDataStruct,
        inner_data_impl: InnerDataImpl,
        mock_struct: MockStruct,
        inner_data_deref_impl: InnerDataDerefImpl,
        trait_mock_impls: Vec<MockTraitImpl>,
        mock_trait_impl: MockTraitImpl,
        mock_impl: MockImpl,
        mock_setup_impl: MockSetupImpl,
        mock_received_impl: MockReceivedImpl,
        ignored_impls: Vec<ItemImpl>,
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
            .chain(self.convert_fn_infos(fn_infos))
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
        let item_mod = self.create_item_mod(ident, items);
        let use_generated_mod = self.create_use_generated_mod(item_mod.ident.clone());
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
            .chain(self.convert_fn_info(fn_info))
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
        let item_mod = self.create_item_mod(fn_ident.clone(), items);
        let use_generated_mod = self.create_use_generated_mod(item_mod.ident.clone());
        let generated_mod = GeneratedMod {
            item_mod,
            use_generated_mod,
        };
        return generated_mod;
    }

    fn generate_struct(
        &self,
        struct_ident: Ident,
        mock_struct_traits: Vec<MockStructTrait>,
        struct_fn_infos: Vec<FnInfo>,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        inner_data_struct: InnerDataStruct,
        inner_data_impl: InnerDataImpl,
        mock_struct: MockStruct,
        inner_data_deref_impl: InnerDataDerefImpl,
        mock_trait_impls: Vec<MockTraitImpl>,
        mock_trait_impl: MockTraitImpl,
        mock_impl: MockImpl,
        mock_setup_impl: MockSetupImpl,
        mock_received_impl: MockReceivedImpl,
        ignored_impls: Vec<ItemImpl>,
    ) -> GeneratedMod {
        let usings = [
            constants::USE_SUPER.clone(),
            constants::USE_FOR_GENERATED.clone(),
        ];
        let ident = format_ident!("{}_{}", Self::GENERATED_MOD_IDENT.clone(), struct_ident);
        let items = usings
            .into_iter()
            .map(|x| Item::Use(x))
            .chain(mock_struct_traits.into_iter().flat_map(|x| {
                self.convert_fn_infos(x.info.fn_infos).into_iter().chain([
                    Item::Struct(x.setup_struct.item_struct),
                    Item::Struct(x.received_struct.item_struct),
                    Item::Impl(x.setup_impl.item_impl),
                    Item::Impl(x.received_impl.item_impl),
                ])
            }))
            .chain(struct_fn_infos.into_iter().flat_map(|x| {
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
                Item::Struct(inner_data_struct.item_struct),
                Item::Impl(inner_data_impl.item_impl),
                Item::Struct(mock_struct.item_struct),
                Item::Impl(inner_data_deref_impl.item_impl),
            ])
            .chain(
                mock_trait_impls
                    .into_iter()
                    .map(|mock_trait_impl| Item::Impl(mock_trait_impl.item_impl)),
            )
            .chain([
                Item::Impl(mock_trait_impl.item_impl),
                Item::Impl(mock_impl.item_impl),
                Item::Impl(mock_setup_impl.item_impl),
                Item::Impl(mock_received_impl.item_impl),
            ])
            .chain(ignored_impls.into_iter().map(Item::Impl))
            .collect();
        let item_mod = self.create_item_mod(ident, items);
        let use_generated_mod = self.create_use_generated_mod(item_mod.ident.clone());
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
    fn convert_fn_info(&self, fn_info: FnInfo) -> Vec<Item> {
        [
            Item::Struct(fn_info.call_struct.item_struct),
            Item::Impl(fn_info.call_arg_infos_provider_impl.item_impl),
            Item::Struct(fn_info.args_checker_struct.item_struct),
            Item::Impl(fn_info.args_checker_impl.item_impl),
        ]
        .into_iter()
        .chain(
            fn_info
                .maybe_base_caller_impl
                .map(|base_caller_impl| Item::Impl(base_caller_impl.item_impl))
                .into_iter(),
        )
        .collect()
    }

    fn convert_fn_infos(&self, fn_infos: Vec<FnInfo>) -> Vec<Item> {
        return fn_infos
            .into_iter()
            .flat_map(|x| self.convert_fn_info(x))
            .collect();
    }

    fn create_item_mod(&self, ident: Ident, items: Vec<Item>) -> ItemMod {
        let item_mod = ItemMod {
            attrs: vec![
                constants::CFG_TEST_ATTRIBUTE.clone(),
                constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone(),
                constants::ALLOW_NON_SNAKE_CASE_ATTRIBUTE.clone(),
                constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone(),
            ],
            vis: Visibility::Inherited,
            unsafety: None,
            mod_token: Default::default(),
            ident,
            content: Some((Default::default(), items)),
            semi: None,
        };
        return item_mod;
    }

    fn create_use_generated_mod(&self, mod_ident: Ident) -> ItemUse {
        let use_generated_mod = ItemUse {
            attrs: vec![constants::CFG_TEST_ATTRIBUTE.clone()],
            vis: Visibility::Public(Default::default()),
            use_token: Default::default(),
            leading_colon: None,
            tree: UseTree::Path(UsePath {
                ident: mod_ident,
                colon2_token: Default::default(),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Default::default(),
                })),
            }),
            semi_token: Default::default(),
        };
        return use_generated_mod;
    }
}
