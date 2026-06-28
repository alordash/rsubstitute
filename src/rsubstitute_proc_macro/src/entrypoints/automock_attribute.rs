use crate::generation::targets;
use crate::preparation;
use syn::*;

// TODO - rename to just `mock`, not `automock`
pub(crate) fn handle(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ctx = preparation::context::create_for_automock_macro(proc_macro_attribute);
    let item = parse_macro_input!(proc_macro_item as Item);
    let r#mod = match item {
        Item::Fn(item_fn) => targets::r#fn::generate_module(ctx, item_fn),
        Item::Impl(item_impl) => todo!(),
        Item::Struct(item_struct) => todo!(),
        Item::Trait(item_trait) => todo!(),
        _ => todo!(
            "PANIC HERE AND WRITE CORRECT ERROR MSG. Can automock only `fn`, `trait`, `impl` or `use`."
        ),
    };
    todo!("return result");

    // TODO - move `use` to `mock!` (or should I? maybe just support Item::Block for that)
    // Should be used as `mock! { core::char::from_u32(i: u32) }
}
