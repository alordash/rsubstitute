use proc_macro::TokenStream;
use syn::{ItemStruct, parse_macro_input};

pub trait IDeriveArgsFormatterMacroHandler {
    fn handle(&self, item: proc_macro::TokenStream) -> proc_macro::TokenStream;
}

pub struct DeriveArgsFormatterMacroHandler;

impl IDeriveArgsFormatterMacroHandler for DeriveArgsFormatterMacroHandler {
    fn handle(&self, item: TokenStream) -> TokenStream {
        let item_struct = parse_macro_input!(item as ItemStruct);
        todo!()
    }
}
