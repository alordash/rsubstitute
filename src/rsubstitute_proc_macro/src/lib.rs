#![allow(clippy::needless_return)]

use crate::automock::handle_automock;
use crate::models::*;

mod automock;
mod constants;
mod models;
mod preparation;
mod syntax;
mod targets;

// TODO - make it work only in test mode.
// basically use `#[cfg(test, mock)]` everywhere (same with `mocked!` ?)
#[proc_macro_attribute]
pub fn automock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    handle_automock(proc_macro_attribute, proc_macro_item)
}

#[proc_macro]
pub fn mock(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mock_core(proc_macro_item, MockMacroUsage::Simple)
}

#[cfg(not(feature = "mock_base_by_default"))]
#[proc_macro]
pub fn mock_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mock_core(proc_macro_item, MockMacroUsage::WithBase)
}

#[cfg(feature = "mock_base_by_default")]
#[proc_macro]
pub fn mock_without_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mock_core(proc_macro_item, MockMacroUsage::WithoutBase)
}

fn mock_core(
    proc_macro_item: proc_macro::TokenStream,
    mock_macro_usage: MockMacroUsage,
) -> proc_macro::TokenStream {
    let context = preparation::create_context_for_mock_macro(mock_macro_usage);
    proc_macro_item
}
