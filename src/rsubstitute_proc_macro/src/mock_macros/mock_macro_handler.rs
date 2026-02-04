use proc_macro::TokenStream;
use syn::parse_macro_input;
use crate::mock_macros::models::StructMockSyntax;

pub trait IMockMacroHandler {
    fn handle(&self, token_stream: TokenStream) -> TokenStream;
}

pub struct MockMacroHandler;

impl IMockMacroHandler for MockMacroHandler {
    fn handle(&self, token_stream: TokenStream) -> TokenStream {
        let struct_mock_syntax = parse_macro_input!(token_stream as StructMockSyntax);
        return TokenStream::new();
    }
}
