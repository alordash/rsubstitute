use crate::di::SERVICES;

mod constants;
mod derive_args_formatter_macro_handler;
mod derive_args_infos_provider_macro_handler;
mod derive_args_tuple_provider_macro_handler;
mod derive_clone_for_r_substitute_macro_handler;
mod derive_generics_hash_key_provider_macro_handler;
mod derive_mock_data_macro_handler;
mod di;
mod mock_macros;
mod syntax;

#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mock_macro_handler = &SERVICES.mock_macro_handler;

    return mock_macro_handler.handle_attribute_macro(proc_macro_attribute, proc_macro_item);
}

#[proc_macro_attribute]
pub fn unmock(
    _proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return proc_macro_item;
}

#[proc_macro]
pub fn mocked(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mock_macro_handler = &SERVICES.mock_macro_handler;

    return mock_macro_handler.handle_macro(proc_macro_item);
}

#[proc_macro_derive(IArgsFormatter)]
pub fn derive_args_formatter(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_args_formatter_macro_handler = &SERVICES.derive_args_formatter_macro_handler;

    return derive_args_formatter_macro_handler.handle(item);
}

#[proc_macro_derive(IArgsInfosProvider)]
pub fn derive_args_infos_provider(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_args_infos_provider_macro_handler =
        &SERVICES.derive_args_infos_provider_macro_handler;

    return derive_args_infos_provider_macro_handler.handle(item);
}

#[proc_macro_derive(IArgsTupleProvider)]
pub fn derive_args_tuple_provider(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_args_tuple_provider_macro_handler =
        &SERVICES.derive_args_tuple_provider_macro_handler;

    return derive_args_tuple_provider_macro_handler.handle(item);
}

#[proc_macro_derive(IMockData)]
pub fn derive_mock_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_mock_data_macro_handler = &SERVICES.derive_mock_data_macro_handler;

    return derive_mock_data_macro_handler.handle(item);
}

#[proc_macro_derive(IGenericsHashKeyProvider)]
pub fn derive_generics_hash_key_provider(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_generics_hash_key_provider_macro_handler =
        &SERVICES.derive_generics_hash_key_provider_macro_handler;

    return derive_generics_hash_key_provider_macro_handler.handle(item);
}

#[proc_macro_derive(CloneForRSubstitute)]
pub fn derive_clone_for_r_substitute(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_clone_for_rsubstitute_macro_handler =
        &SERVICES.derive_clone_for_rsubstitute_macro_handler;

    return derive_clone_for_rsubstitute_macro_handler.handle(item);
}
