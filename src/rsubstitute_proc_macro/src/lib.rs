#![allow(clippy::needless_return)]

use crate::preparation::models::*;

mod constants;
mod models;
mod preparation;

// TODO - make it work only in test mode.
// basically use `#[cfg(test, mock)]` everywhere (same with `mocked!` ?)
#[proc_macro_attribute]
pub fn automock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let context = preparation::create_context_for_automock_macro(proc_macro_attribute);
    proc_macro_item
}

#[proc_macro]
pub fn mock(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let context = preparation::create_context_for_mock_macro(MockMacroUsage::Simple);
    proc_macro_item
}


#[cfg(not(feature = "mock_base_by_default"))]
#[proc_macro]
pub fn mocked_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let context = preparation::create_context_for_mock_macro(MockMacroUsage::WithBase);
    proc_macro_item
}

#[cfg(feature = "mock_base_by_default")]
#[proc_macro]
pub fn mocked_no_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let context = preparation::create_context_for_mock_macro(MockMacroUsage::WithoutBase);
    proc_macro_item
}