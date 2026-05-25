use crate::{preparation, targets};
use syn::*;

// TODO - rename to just `mock`, not `automock`
pub(crate) fn handle_automock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ctx = preparation::create_context_for_automock_macro(proc_macro_attribute);
    let item = parse_macro_input!(proc_macro_item as Item);
    let result = match item {
        Item::Fn(item_fn) => targets::handle_fn(ctx, item_fn),
        Item::Impl(item_impl) => targets::handle_impl(ctx, item_impl),
        Item::Struct(item_struct) => targets::handle_struct(ctx, item_struct),
        Item::Trait(item_trait) => targets::handle_trait(ctx, item_trait),
        _ => todo!("PANIC HERE AND WRITE CORRECT ERROR MSG. Can automock only `fn`, `trait` or `use`.")
    };
    todo!("return result");

    // TODO - move `use` to `mock!` (or should I? maybe just support Item::Block for that)
    // Should be used as `mock! { core::char::from_u32(i: u32) }
}
