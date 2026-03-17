use crate::mock_generation::models::StructMockSyntax;
use crate::mock_generation::parameters::*;
use crate::mock_generation::targets::*;
use crate::mock_generation::*;
use proc_macro::TokenStream;
use syn::*;

pub(crate) fn handle_attribute_macro(
    proc_macro_attribute: TokenStream,
    proc_macro_item: TokenStream,
) -> TokenStream {
    let ctx = ctx::create(proc_macro_attribute);
    if let Ok(item_trait) = syn::parse::<ItemTrait>(proc_macro_item.clone()) {
        return trait_mock::handle(&ctx, item_trait);
    } else if let Ok(item_fn) = syn::parse::<ItemFn>(proc_macro_item) {
        return fn_mock::handle(&ctx, item_fn);
    }

    panic!("Expected `trait`, `impl` or `fn`.");
}

pub(crate) fn handle_macro_mocked(
    token_stream: TokenStream,
    mocked_macro_mode: MockedMacroMode,
) -> TokenStream {
    let ctx = ctx::create_for_macro_mocked(mocked_macro_mode);
    let struct_mock_syntax = parse_macro_input!(token_stream as StructMockSyntax);
    return struct_mock::handle(&ctx, struct_mock_syntax);
}
