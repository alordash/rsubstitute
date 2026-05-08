#![allow(clippy::needless_return)]

// TODO - make it work only in test mode.
// basically use `#[cfg(test, mock)]` everywhere (same with `mocked!` ?)
#[proc_macro_attribute]
pub fn automock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_item
}

#[proc_macro]
pub fn mock(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_item
}
