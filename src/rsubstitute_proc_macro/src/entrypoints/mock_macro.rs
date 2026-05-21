use crate::models::MockMacroUsage;
use crate::preparation;

pub(crate) fn handle_mock(
    token_stream: proc_macro::TokenStream,
    mock_macro_usage: MockMacroUsage,
) -> proc_macro::TokenStream {
    let context = preparation::create_context_for_mock_macro(mock_macro_usage);
    // TODO - parse using `impl Parse for StructMockSyntax/UseMockSyntax`
    // Create custom struct and impl syn trait `Parse` for it
    // Then: let struct_mock_syntax = parse_macro_input!(token_stream as StructMockSyntax);

    // TODO - move `use` to `mock!`
    // Should be used as `mock! { core::char::from_u32(i: u32) }
    panic!("Can automock only `fn`, `trait` or `use`.");
}
