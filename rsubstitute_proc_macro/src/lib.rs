#![allow(clippy::needless_return)]

mod common;
mod constants;
mod entrypoints;
mod generation;
mod preparation;
mod syntax;

#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    entrypoints::mock_attribute::handle(proc_macro_attribute, proc_macro_item)
}

// TODO - write warning (?) if applying `#[mock(base)]` instead of just `#[mock]` on `struct`
