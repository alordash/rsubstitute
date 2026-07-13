mod mockable_trait;

use crate::common::models::*;
use crate::generation::targets::common::*;
use crate::generation::targets::models::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, item_struct: ItemStruct) -> MockMod {
    let source_span = item_struct.span();
    let mockable_trait = mockable_trait::generate(source_span);

    let mod_visibility = item_struct.vis.clone();
    let mod_ident = format_ident!("__rsubstitute_generated_{}Mock", item_struct.ident);
    let usage = mod_usage::new(mod_ident.clone(), [item_struct.ident.clone()]);
    let items = vec![Item::Struct(item_struct), Item::Trait(mockable_trait)];
    let item_mod = ItemMod {
        attrs: Vec::new(),
        vis: mod_visibility,
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod { usage, item_mod };
    return result;
}
