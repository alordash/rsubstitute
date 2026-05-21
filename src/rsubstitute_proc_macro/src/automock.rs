use crate::models::Context;
use crate::preparation::r#fn::*;
use crate::{preparation, targets};
use syn::*;

pub(crate) fn handle_automock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let context = preparation::create_context_for_automock_macro(proc_macro_attribute);
    let maybe_item_fn = syn::parse::<ItemFn>(proc_macro_item.clone());
    if let Ok(item_fn) = maybe_item_fn {
        let result = targets::handle_fn(item_fn);
        return todo!("result");
    }
    let maybe_trait_item = syn::parse::<ItemTrait>(proc_macro_item.clone());
    if let Ok(trait_item) = maybe_trait_item {
        let result = todo!("parse_trait_syntax");
        return todo!("result");
    }
    let maybe_use_item = syn::parse::<ItemUse>(proc_macro_item);
    if let Ok(use_item) = maybe_use_item {
        let result = todo!("parse_use_item");
        return todo!("result");
    }

    // TODO - move `use` to `mock!`
    // Should be used as `mock! { core::char::from_u32(i: u32) }
    panic!("Can automock only `fn`, `trait` or `use`.");
}
