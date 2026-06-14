#![allow(clippy::needless_return)]

mod constants;
mod entrypoints;
mod generation;
mod preparation;
mod syntax;
mod targets;

use crate::entrypoints::models::*;

// TODO - make it work only in test mode.
// basically use `#[cfg(test, mock)]` everywhere (same with `mocked!` ?)
#[proc_macro_attribute]
pub fn automock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    entrypoints::automock_attribute::handle(proc_macro_attribute, proc_macro_item)
}

#[proc_macro]
pub fn mock(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entrypoints::mock_macro::handle(proc_macro_item, MockMacroUsage::Simple)
}

#[cfg(not(feature = "mock_base_by_default"))]
#[proc_macro]
pub fn mock_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entrypoints::mock_macro::handle(proc_macro_item, MockMacroUsage::WithBase)
}

#[cfg(feature = "mock_base_by_default")]
#[proc_macro]
pub fn mock_without_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entrypoints::mock_macro::handle(proc_macro_item, MockMacroUsage::WithoutBase)
}
