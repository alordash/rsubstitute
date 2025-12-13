use crate::macros::fn_info_extractor::IFnInfoExtractor;
use proc_macro::TokenStream;
use std::rc::Rc;
use std::sync::Arc;
use syn::{ItemImpl, ItemTrait, parse_macro_input};

pub trait IMacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;
}

pub struct MacroHandler {
    pub(crate) fn_info_extractor: Rc<dyn IFnInfoExtractor>,
}

impl IMacroHandler for MacroHandler {
    fn handle(
        &self,
        _proc_macro_attribute: TokenStream,
        proc_macro_item: TokenStream,
    ) -> TokenStream {
        if let Ok(item_impl) = syn::parse::<ItemImpl>(proc_macro_item.clone()) {
            return self.handle_item_impl(item_impl);
        } else if let Ok(item_trait) = syn::parse::<ItemTrait>(proc_macro_item) {
            return self.handle_item_trait(item_trait);
        }

        panic!("Expected `impl` or `trait`.");
    }
}

impl MacroHandler {
    fn handle_item_impl(&self, item_impl: ItemImpl) -> TokenStream {
        todo!();
    }

    fn handle_item_trait(&self, item_trait: ItemTrait) -> TokenStream {
        let fn_infos = self.fn_info_extractor.extract(item_trait.items);
        todo!();
    }
}
