use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::{
    InternalMockImpl, InternalMockReceivedImpl, InternalMockSetupImpl, MockDataStruct, MockImpl,
    MockReceivedStruct, MockSetupStruct, MockStruct,
};
use crate::mock_macros::models::GeneratedMod;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use syn::{Item, ItemMod, Visibility};

pub trait IModGenerator {
    fn generate(
        &self,
        trait_ident: Ident,
        fn_infos: Vec<FnInfo>,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        mock_struct: MockStruct,
        mock_impl: MockImpl,
        internal_mock_impl: InternalMockImpl,
        internal_mock_setup_impl: InternalMockSetupImpl,
        internal_mock_received_impl: InternalMockReceivedImpl,
    ) -> GeneratedMod;
}

pub struct ModGenerator;

impl IModGenerator for ModGenerator {
    fn generate(
        &self,
        trait_ident: Ident,
        fn_infos: Vec<FnInfo>,
        mock_data_struct: MockDataStruct,
        mock_setup_struct: MockSetupStruct,
        mock_received_struct: MockReceivedStruct,
        mock_struct: MockStruct,
        mock_impl: MockImpl,
        internal_mock_impl: InternalMockImpl,
        internal_mock_setup_impl: InternalMockSetupImpl,
        internal_mock_received_impl: InternalMockReceivedImpl,
    ) -> GeneratedMod {
        let attrs = vec![constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone()];
        let usings = [
            constants::USE_SUPER.clone(),
            constants::USE_CRATE_PRELUDE.clone(),
            constants::USE_STD_MARKER_PHANTOM_DATA.clone(),
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
                Item::Impl(mock_impl.item_impl),
                Item::Impl(internal_mock_impl.item_impl),
                Item::Impl(internal_mock_setup_impl.item_impl),
                Item::Impl(internal_mock_received_impl.item_impl),
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
        let mod_info = GeneratedMod { item_mod };
        return mod_info;
    }
}

impl ModGenerator {
    const GENERATED_MOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("__rsubstitute_generated"));
}
