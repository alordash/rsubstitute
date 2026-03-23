use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use syn::*;

pub(crate) fn generate_trait(
    trait_ident: Ident,
    fn_infos: Vec<FnInfo>,
    mock_data_struct: MockDataStruct,
    mock_setup_struct: MockSetupStruct,
    mock_received_struct: MockReceivedStruct,
    mock_struct: MockStruct,
    mock_trait_impl: MockPayloadImpl,
    mock_impl: MockImpl,
    mock_setup_impl: MockSetupImpl,
    mock_received_impl: MockReceivedImpl,
) -> GeneratedMod {
    let usings = [
        constants::USE_SUPER.clone(),
        constants::USE_FOR_GENERATED.clone(),
    ];
    let ident = format_ident!("{}_{}", GENERATED_MOD_IDENT.clone(), trait_ident);
    let items = usings
        .into_iter()
        .map(|x| Item::Use(x))
        .chain(convert_fn_infos(fn_infos))
        .chain([Item::Struct(mock_data_struct.item_struct)])
        .chain(convert_mock_setup_struct(mock_setup_struct))
        .chain(convert_mock_received_struct(mock_received_struct))
        .chain(convert_mock_struct(mock_struct))
        .chain([
            Item::Impl(mock_trait_impl.item_impl),
            Item::Impl(mock_impl.item_impl),
            Item::Impl(mock_setup_impl.item_impl),
            Item::Impl(mock_received_impl.item_impl),
        ])
        .collect();
    let item_mod = create_item_mod(ident, items);
    let use_generated_mod = create_use_generated_mod(item_mod.ident.clone());
    let generated_mod = GeneratedMod {
        item_mod,
        use_generated_mod,
    };
    return generated_mod;
}

pub(crate) fn generate_fn(
    fn_ident: Ident,
    fn_info: FnInfo,
    mock_data_struct: MockDataStruct,
    mock_setup_struct: MockSetupStruct,
    mock_received_struct: MockReceivedStruct,
    mock_struct: MockStruct,
    mock_struct_default_impl: MockStructDefaultImpl,
    mock_setup_impl: MockSetupImpl,
    mock_received_impl: MockReceivedImpl,
    fn_get_mock: ItemFn,
    fn_setup: ItemFn,
    fn_received: ItemFn,
    static_fn: StaticFn,
    maybe_static_base_fn: Option<StaticBaseFn>,
) -> GeneratedMod {
    let usings = [
        constants::USE_SUPER.clone(),
        constants::USE_FOR_GENERATED.clone(),
    ];
    let items = usings
        .into_iter()
        .map(|x| Item::Use(x))
        .chain(convert_fn_info(fn_info))
        .chain([Item::Struct(mock_data_struct.item_struct)])
        .chain(convert_mock_setup_struct(mock_setup_struct))
        .chain(convert_mock_received_struct(mock_received_struct))
        .chain(convert_mock_struct(mock_struct))
        .chain([
            Item::Impl(mock_struct_default_impl.item_impl),
            Item::Impl(mock_setup_impl.item_impl),
            Item::Impl(mock_received_impl.item_impl),
            Item::Fn(fn_get_mock),
            Item::Fn(fn_setup),
            Item::Fn(fn_received),
            Item::Fn(static_fn.item_fn),
        ])
        .chain(maybe_static_base_fn.map(|static_base_fn| Item::Fn(static_base_fn.item_fn)))
        .collect();
    let item_mod = create_item_mod(fn_ident.clone(), items);
    let use_generated_mod = create_use_generated_mod(item_mod.ident.clone());
    let generated_mod = GeneratedMod {
        item_mod,
        use_generated_mod,
    };
    return generated_mod;
}

pub(crate) fn generate_struct(
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
    mock_trait_impls: Vec<MockPayloadImpl>,
    mock_struct_impl: MockPayloadImpl,
    mock_impl: MockImpl,
    mock_setup_impl: MockSetupImpl,
    mock_received_impl: MockReceivedImpl,
    ignored_impls: Vec<ItemImpl>,
) -> GeneratedMod {
    let usings = [
        constants::USE_SUPER.clone(),
        constants::USE_FOR_GENERATED.clone(),
    ];
    let ident = format_ident!("{}_{}", GENERATED_MOD_IDENT.clone(), struct_ident);
    let items = usings
        .into_iter()
        .map(|x| Item::Use(x))
        .chain(mock_struct_traits.into_iter().flat_map(|x| {
            convert_fn_infos(x.info.fn_infos).into_iter().chain(
                [
                    convert_mock_setup_struct(x.setup_struct),
                    convert_mock_received_struct(x.received_struct),
                ]
                .into_iter()
                .flatten()
                .chain([
                    Item::Impl(x.setup_impl.item_impl),
                    Item::Impl(x.received_impl.item_impl),
                ]),
            )
        }))
        .chain(struct_fn_infos.into_iter().flat_map(|x| convert_fn_info(x)))
        .chain([Item::Struct(mock_data_struct.item_struct)])
        .chain(convert_mock_setup_struct(mock_setup_struct))
        .chain(convert_mock_received_struct(mock_received_struct))
        .chain([
            Item::Struct(inner_data_struct.item_struct),
            Item::Impl(inner_data_impl.item_impl),
        ])
        .chain(convert_mock_struct(mock_struct))
        .chain([Item::Impl(inner_data_deref_impl.item_impl)])
        .chain(
            mock_trait_impls
                .into_iter()
                .map(|mock_trait_impl| Item::Impl(mock_trait_impl.item_impl)),
        )
        .chain([
            Item::Impl(mock_struct_impl.item_impl),
            Item::Impl(mock_impl.item_impl),
            Item::Impl(mock_setup_impl.item_impl),
            Item::Impl(mock_received_impl.item_impl),
        ])
        .chain(ignored_impls.into_iter().map(Item::Impl))
        .collect();
    let item_mod = create_item_mod(ident, items);
    let use_generated_mod = create_use_generated_mod(item_mod.ident.clone());
    let generated_mod = GeneratedMod {
        item_mod,
        use_generated_mod,
    };
    return generated_mod;
}

const GENERATED_MOD_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("__rsubstitute_generated"));

fn convert_fn_infos(fn_infos: Vec<FnInfo>) -> Vec<Item> {
    return fn_infos
        .into_iter()
        .flat_map(|x| convert_fn_info(x))
        .collect();
}

fn convert_fn_info(fn_info: FnInfo) -> Vec<Item> {
    [
        Item::Struct(fn_info.call_struct.item_struct),
        Item::Impl(fn_info.call_struct.args_infos_provider_trait_impl),
        Item::Impl(fn_info.call_struct.args_tuple_provider_trait_impl),
        Item::Impl(fn_info.call_struct.generics_info_provider_impl),
    ]
    .into_iter()
    .chain(
        fn_info
            .call_struct
            .maybe_clone_for_rsubstitute_trait_impl
            .map(Item::Impl)
            .into_iter(),
    )
    .chain([
        Item::Struct(fn_info.args_checker_struct.item_struct),
        Item::Impl(fn_info.args_checker_struct.args_checker_trait_impl),
        Item::Impl(
            fn_info
                .args_checker_struct
                .args_checker_args_formatter_trait_impl,
        ),
        Item::Impl(fn_info.args_checker_struct.generics_info_provider_impl),
    ])
    .collect()
}

fn convert_mock_setup_struct(mock_setup_struct: MockSetupStruct) -> [Item; 2] {
    [
        Item::Struct(mock_setup_struct.item_struct),
        Item::Impl(mock_setup_struct.clone_for_rsubstitute_trait_impl),
    ]
}

fn convert_mock_received_struct(mock_received_struct: MockReceivedStruct) -> [Item; 2] {
    [
        Item::Struct(mock_received_struct.item_struct),
        Item::Impl(mock_received_struct.clone_for_rsubstitute_trait_impl),
    ]
}

fn convert_mock_struct(mock_struct: MockStruct) -> Vec<Item> {
    [
        Item::Struct(mock_struct.item_struct),
        Item::Impl(mock_struct.as_ref_trait_impl),
    ]
    .into_iter()
    .chain(
        mock_struct
            .maybe_clone_for_rsubstitute_trait_impl
            .map(Item::Impl)
            .into_iter(),
    )
    .collect()
}

fn create_item_mod(ident: Ident, items: Vec<Item>) -> ItemMod {
    let item_mod = ItemMod {
        attrs: vec![
            constants::CFG_TEST_ATTRIBUTE.clone(),
            constants::ALLOW_UNUSED_PARENS_ATTRIBUTE.clone(),
            constants::ALLOW_NON_SNAKE_CASE_ATTRIBUTE.clone(),
            constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone(),
            constants::ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE.clone(),
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

fn create_use_generated_mod(mod_ident: Ident) -> ItemUse {
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
