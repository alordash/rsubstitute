use crate::mock_macros::models::StructMockSyntax;
use proc_macro::TokenStream;
use syn::parse_macro_input;

pub trait IMockMacroHandler {
    fn handle(&self, token_stream: TokenStream) -> TokenStream;
}

pub struct MockMacroHandler;

impl IMockMacroHandler for MockMacroHandler {
    fn handle(&self, token_stream: TokenStream) -> TokenStream {
        let source_token_stream = token_stream.clone();
        let struct_mock_syntax = parse_macro_input!(token_stream as StructMockSyntax);
        dbg!(struct_mock_syntax);
        return source_token_stream;
    }
}
