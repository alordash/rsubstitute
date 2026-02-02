use proc_macro::TokenStream;

pub trait IMockMacroHandler {
    fn handle(&self, token_stream: TokenStream) -> TokenStream;
}

pub struct MockMacroHandler;

impl IMockMacroHandler for MockMacroHandler {
    fn handle(&self, token_stream: TokenStream) -> TokenStream {
        return TokenStream::new();
        todo!()
    }
}
