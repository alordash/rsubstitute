use crate::preparation::r#trait::*;
use syn::*;
use crate::models::Context;

pub(crate) fn handle_trait(ctx: Context, item_trait: ItemTrait) {
    let syntax = prepare_trait_syntax(PrepareTraitFnSyntaxArgs {
        attributes: item_trait.attrs,
        visibility: item_trait.vis,
        ident: item_trait.ident,
        generics: item_trait.generics,
        items: item_trait.items,
    });
}
