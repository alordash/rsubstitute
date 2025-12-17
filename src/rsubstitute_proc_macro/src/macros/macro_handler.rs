use crate::macros::fn_decl_extractor::IFnDeclExtractor;
use proc_macro::TokenStream;
use std::rc::Rc;
use syn::{ItemImpl, ItemTrait};

pub trait IMacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;
}

pub struct MacroHandler {
    pub(crate) fn_decl_extractor: Rc<dyn IFnDeclExtractor>,
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
        let fn_decls = self.fn_decl_extractor.extract(item_trait.items);
        todo!();
    }
}
