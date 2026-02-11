use crate::mock_macros::models::StructMockSyntax;
use crate::mock_macros::targets::{IItemFnHandler, IItemTraitHandler, IStructMockHandler};
use proc_macro::TokenStream;
use std::sync::Arc;
use syn::{ItemFn, ItemTrait, parse_macro_input};

pub trait IMockMacroHandler {
    fn handle_attribute_macro(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;

    fn handle_macro(&self, token_stream: TokenStream) -> TokenStream;
}

pub(crate) struct MockMacroHandler {
    pub item_trait_handler: Arc<dyn IItemTraitHandler>,
    pub item_fn_handler: Arc<dyn IItemFnHandler>,
    pub struct_mock_handler: Arc<dyn IStructMockHandler>,
}

impl IMockMacroHandler for MockMacroHandler {
    fn handle_attribute_macro(
        &self,
        _proc_macro_attribute: TokenStream,
        proc_macro_item: TokenStream,
    ) -> TokenStream {
        if let Ok(item_trait) = syn::parse::<ItemTrait>(proc_macro_item.clone()) {
            return self.item_trait_handler.handle(item_trait);
        } else if let Ok(item_fn) = syn::parse::<ItemFn>(proc_macro_item) {
            return self.item_fn_handler.handle(item_fn);
        }

        panic!("Expected `trait`, `impl` or `fn`.");
    }

    fn handle_macro(&self, token_stream: TokenStream) -> TokenStream {
        let struct_mock_syntax = parse_macro_input!(token_stream as StructMockSyntax);
        return self.struct_mock_handler.handle(struct_mock_syntax);
    }
}
