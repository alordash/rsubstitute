use crate::common::*;
use crate::generation::targets;
use quote::quote;
use syn::spanned::Spanned;
use syn::*;

// TODO - rename to just `mock`, not `automock`
pub(crate) fn handle(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ctx = context::create_for_automock_macro(proc_macro_attribute);
    let item = parse_macro_input!(proc_macro_item as Item);
    let source_span = item.span();
    let mock_mod = match item {
        Item::Fn(item_fn) => targets::r#fn::generate_module(&ctx, item_fn),
        Item::Impl(item_impl) => todo!(),
        Item::Struct(item_struct) => todo!(),
        Item::Trait(item_trait) => todo!(),
        _ => todo!(
            "PANIC HERE AND WRITE CORRECT ERROR MSG. Can automock only `fn`, `trait`, `impl` or `use`."
        ),
    };

    let use_mock_mod = ItemUse {
        attrs: Vec::new(),
        vis: mock_mod.visibility,
        use_token: Token![use](source_span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: mock_mod.item_mod.ident.clone(),
            colon2_token: Token![::](source_span),
            tree: Box::new(UseTree::Glob(UseGlob {
                star_token: Token![*](source_span),
            })),
        }),
        semi_token: Token![;](source_span),
    };
    let r#mod = mock_mod.item_mod;
    let result = quote! {
        #use_mock_mod
        #r#mod
    };
    return result.into();

    // TODO - move `use` to `mock!` (or should I? maybe just support Item::Block for that)
    // Should be used as `mock! { core::char::from_u32(i: u32) }
}
