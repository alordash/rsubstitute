use crate::preparation::models::*;
use crate::preparation::r#trait::*;
use syn::*;

pub(crate) fn handle(ctx: Context, item_trait: ItemTrait) {
    let syntax = trait_syntax::prepare(trait_syntax::Params {
        attributes: item_trait.attrs,
        visibility: item_trait.vis,
        ident: item_trait.ident,
        generics: item_trait.generics,
        items: item_trait.items,
    });
    todo!()
}
