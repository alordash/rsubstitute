use crate::common::*;
use crate::generation::targets;
use crate::generation::targets::models::*;
use quote::quote;
use syn::*;

pub(crate) fn handle(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ctx = context::create_for_mock_macro(proc_macro_attribute);
    let item = parse_macro_input!(proc_macro_item as Item);
    let mock_mod = match item {
        Item::Fn(item_fn) => targets::r#fn::generate_module(&ctx, item_fn),
        Item::Impl(item_impl) => todo!(),
        Item::Struct(item_struct) => todo!(),
        Item::Trait(item_trait) => targets::r#trait::generate_module(&ctx, item_trait),
        _ => todo!(
            "PANIC HERE AND WRITE CORRECT ERROR MSG. Can automock only `fn`, `trait`, `impl` or `use`."
        ),
    };

    let MockMod { usage, item_mod } = mock_mod;
    let result = quote! {
        #usage
        #item_mod
    };
    return result.into();

    // TODO - move `use` to `mock!` (or should I? maybe just support Item::Block for that)
    // Should be used as `mock! { core::char::from_u32(i: u32) }
}
