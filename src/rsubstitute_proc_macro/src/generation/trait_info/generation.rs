use crate::common::models::*;
use crate::generation::common::models::*;
use crate::generation::fn_info;
use crate::generation::fn_info::models::*;
use crate::generation::trait_info::models::*;
use crate::preparation::common::models::*;
use crate::preparation::r#fn::models::*;
use crate::preparation::r#trait::models::*;
use crate::syntax::generic_argument;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(ctx: &Context, trait_syntax: TraitSyntax) -> TraitInfo {
    let mock_struct_ident = format_ident!("{}Mock", trait_syntax.ident);
    let span = trait_syntax.ident.span();
    let associated_items_info = AssociatedItemsInfo {
        trait_path_segment: PathSegment {
            ident: trait_syntax.ident.clone(),
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: Some(Token![::](span)),
                lt_token: Token![<](span),
                args: trait_syntax
                    .merged_generics
                    .params
                    .iter()
                    .map(|x| generic_argument::from_param(x.clone()))
                    .collect(),
                gt_token: Token![>](span),
            }),
        },
        associated_items_ident_strings: trait_syntax
            .constants
            .iter()
            .map(|x| x.item.ident.to_string())
            .chain(
                trait_syntax
                    .assoc_types
                    .iter()
                    .map(|x| x.item.ident.to_string()),
            )
            .collect(),
    };
    let result = TraitInfo {
        attributes: trait_syntax.attributes,
        unsafety: trait_syntax.unsafety,
        visibility: trait_syntax.visibility,
        ident: trait_syntax.ident,
        merged_generics: trait_syntax.merged_generics,
        constants: trait_syntax.constants,
        assoc_types: trait_syntax.assoc_types,
        path: trait_syntax.path,
        static_fns: convert_fns(ctx, trait_syntax.static_fns),
        associated_fns: convert_fns(ctx, trait_syntax.associated_fns),
        mock_struct_ident,
        associated_items_info,
    };
    return result;
}

fn convert_fns(ctx: &Context, fn_syntaxes: Vec<Ordered<FnSyntax>>) -> Vec<Ordered<FnInfo>> {
    fn_syntaxes
        .into_iter()
        .map(|ordered_fn_syntax| ordered_fn_syntax.map(|x| fn_info::generate(ctx, x)))
        .collect()
}
