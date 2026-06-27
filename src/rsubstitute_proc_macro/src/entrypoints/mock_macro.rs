use super::models::*;
use crate::preparation;

pub(crate) fn handle(
    token_stream: proc_macro::TokenStream,
    mock_macro_usage: MockMacroUsage,
) -> proc_macro::TokenStream {
    let context = preparation::context::create_for_mock_macro(mock_macro_usage);
    // THIS MAY BE NOT THE BEST APPROACH!
    // IIRC there was some method in parse stream lik `peek` that returns what is current syn item
    // TODO - parse using `impl Parse for StructMockSyntax/UseMockSyntax`
    // Create custom trait and impl syn trait `Parse` for it
    // Then: let struct_mock_syntax = parse_macro_input!(token_stream as StructMockSyntax);

    // TODO - move `use` to `mock!`
    // Should be used as `mock! { core::char::from_u32(i: u32) }
    panic!("Can automock only `fn_info`, `trait` or `use`.");
}
