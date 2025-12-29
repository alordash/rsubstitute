use crate::mock_macros::targets::IItemTraitHandler;
use proc_macro::TokenStream;
use std::sync::Arc;
use syn::*;

pub trait IMockMacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;
}

pub(crate) struct MockMacroHandler {
    pub item_trait_handler: Arc<dyn IItemTraitHandler>,
}

impl IMockMacroHandler for MockMacroHandler {
    fn handle(
        &self,
        _proc_macro_attribute: TokenStream,
        proc_macro_item: TokenStream,
    ) -> TokenStream {
        if let Ok(item_impl) = syn::parse::<ItemImpl>(proc_macro_item.clone()) {
            return self.handle_item_impl(item_impl);
        } else if let Ok(item_trait) = syn::parse::<ItemTrait>(proc_macro_item) {
            return self.item_trait_handler.handle(item_trait);
        }

        panic!("Expected `impl` or `trait`.");
    }
}

impl MockMacroHandler {
    fn handle_item_impl(&self, _item_impl: ItemImpl) -> TokenStream {
        todo!();
    }
}
