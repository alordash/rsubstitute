#![allow(clippy::needless_return)]
use crate::mock_generation::parameters::*;
use crate::mock_generation::*;

mod constants;
mod derive_mock_data_macro;
mod mock_generation;
mod syntax;

// TODO - make it work only in test mode.
// basically use `#[cfg(test, mock)]` everywhere (same with `mocked!` ?)
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

#[proc_macro_derive(IMockData)]
pub fn derive_mock_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_mock_data_macro::handle(item)
}
