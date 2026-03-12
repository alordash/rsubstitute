#![allow(clippy::needless_return)]
use crate::mock_generation::parameters::*;
use crate::mock_generation::*;

mod constants;
mod derive_args_formatter_macro;
mod derive_args_infos_provider_macro;
mod derive_args_tuple_provider_macro;
mod derive_clone_for_rsubstitute_macro;
mod derive_generics_hash_key_provider_macro;
mod derive_mock_data_macro;
mod mock_generation;
mod syntax;

#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    mock_macro::handle_attribute_macro(proc_macro_attribute, proc_macro_item)
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
    mock_macro::handle_macro_mocked(proc_macro_item, MockedMacroMode::Unspecified)
}

#[cfg(not(feature = "mock_base_by_default"))]
#[proc_macro]
pub fn mocked_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mock_macro::handle_macro_mocked(proc_macro_item, MockedMacroMode::WithBase)
}

#[cfg(feature = "mock_base_by_default")]
#[proc_macro]
pub fn mocked_no_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mock_macro_handler.handle_macro_mocked(proc_macro_item, MockedMacroMode::WithoutBase)
}

#[proc_macro_derive(IArgsFormatter)]
pub fn derive_args_formatter(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_args_formatter_macro::handle(item)
}

#[proc_macro_derive(IArgsInfosProvider)]
pub fn derive_args_infos_provider(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_args_infos_provider_macro::handle(item)
}

#[proc_macro_derive(IArgsTupleProvider)]
pub fn derive_args_tuple_provider(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_args_tuple_provider_macro::handle(item)
}

#[proc_macro_derive(IMockData)]
pub fn derive_mock_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_mock_data_macro::handle(item)
}

#[proc_macro_derive(IGenericsHashKeyProvider)]
pub fn derive_generics_hash_key_provider(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_generics_hash_key_provider_macro::handle(item)
}

#[proc_macro_derive(CloneForRSubstitute)]
pub fn derive_clone_for_r_substitute(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_clone_for_rsubstitute_macro::handle(item)
}
