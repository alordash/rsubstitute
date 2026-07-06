use crate::common::models::*;
use crate::generation::fn_info;
use crate::generation::trait_info::models::*;
use crate::preparation::r#trait::models::*;

pub(crate) fn generate(ctx: &Context, trait_syntax: TraitSyntax) -> TraitInfo {
    let result = TraitInfo {
        attributes: trait_syntax.attributes,
        unsafety: trait_syntax.unsafety,
        visibility: trait_syntax.visibility,
        ident: trait_syntax.ident,
        merged_generics: trait_syntax.merged_generics,
        constants: trait_syntax.constants,
        assoc_types: trait_syntax.assoc_types,
        path: trait_syntax.path,
        methods: trait_syntax
            .methods
            .into_iter()
            .map(|ordered_fn_syntax| ordered_fn_syntax.map(|x| fn_info::generate(ctx, x)))
            .collect(),
    };
    return result;
}
