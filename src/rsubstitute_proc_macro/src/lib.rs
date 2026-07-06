#![allow(clippy::needless_return)]

mod common;
mod constants;
mod entrypoints;
mod generation;
mod preparation;
mod syntax;

// TODO - make it work only in test mode.
// basically use `#[cfg(test, mock)]` everywhere (same with `mocked!` ?)
#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    entrypoints::mock_attribute::handle(proc_macro_attribute, proc_macro_item)
}

// #[proc_macro]
// pub fn mock(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     entrypoints::mock_macro::handle(proc_macro_item, MockMacroUsage::Simple)
// }
//
// #[cfg(not(feature = "mock_base_by_default"))]
// #[proc_macro]
// pub fn mock_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     entrypoints::mock_macro::handle(proc_macro_item, MockMacroUsage::WithBase)
// }
//
// #[cfg(feature = "mock_base_by_default")]
// #[proc_macro]
// pub fn mock_without_base(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     entrypoints::mock_macro::handle(proc_macro_item, MockMacroUsage::WithoutBase)
// }

// TODO - write test for this:
// mod a { pub trait Trait {} }
// mod b { pub trait Trait {} }
// mod c { #[mock] pub struct S; }
// #[mock]
// impl a::Trait for c::S {}
// #[mock]
// impl b::Trait for c::S {}

// TODO - write warning (?) if applying `#[mock(base)]` instead of just `#[mock]` on `struct`
