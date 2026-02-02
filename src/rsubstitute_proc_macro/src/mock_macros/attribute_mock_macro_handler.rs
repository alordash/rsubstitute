use crate::mock_macros::targets::{IItemFnHandler, IItemTraitHandler};
use proc_macro::TokenStream;
use std::sync::Arc;
use syn::*;

pub trait IAttributeMockMacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;
}

pub(crate) struct AttributeMockMacroHandler {
    pub item_trait_handler: Arc<dyn IItemTraitHandler>,
    pub item_fn_handler: Arc<dyn IItemFnHandler>,
}

impl IAttributeMockMacroHandler for AttributeMockMacroHandler {
    fn handle(
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
}
