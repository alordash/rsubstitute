use proc_macro::TokenStream;
use syn::{ItemImpl, parse_macro_input};

pub trait IMacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;
}

pub struct MacroHandler;

impl IMacroHandler for MacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: TokenStream,
        proc_macro_item: TokenStream,
    ) -> TokenStream {
        let item_impl = parse_macro_input!(proc_macro_item as ItemImpl);

        todo!()
    }
}

impl MacroHandler {}
