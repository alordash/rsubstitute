use crate::common::models::*;
use crate::generation::mock_struct::*;
use crate::generation::targets::models::*;
use crate::preparation::r#trait::*;
use crate::syntax::*;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, item_trait: ItemTrait) -> MockMod {
    let source_span = item_trait.span();
    let trait_syntax = trait_syntax::prepare(trait_syntax::Params {
        attributes: item_trait.attrs.clone(),
        visibility: item_trait.vis.clone(),
        ident: item_trait.ident.clone(),
        generics: item_trait.generics.clone(),
        items: item_trait.items.clone(),
    });

    let trait_mock_struct = trait_mock_struct::generate(source_span, trait_syntax);

    let mod_visibility = item_trait.vis.clone();
    let items = [Item::Trait(item_trait)].into_iter().collect();

    let usage = todo!();
    let item_mod = ItemMod {
        attrs: vec![attributes::allow_non_camel_case_types(source_span)],
        vis: mod_visibility,
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: todo!(),
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod { usage, item_mod };
    return result;
}
