use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate(source_span: Span, target_path: &Path) -> ItemUse {
    let tree_tail = UseTree::Path(UsePath {
        ident: format_ident!(
            "__rsubstitute_generated_{}Mock",
            path::last_ident(target_path)
        ),
        colon2_token: Token![::](source_span),
        tree: Box::new(UseTree::Glob(UseGlob {
            star_token: Token![*](source_span),
        })),
    });
    let tree_body_and_tail = target_path
        .segments
        .iter()
        .take(target_path.segments.len() - 1)
        .rev()
        .fold(tree_tail, |tree, segment| {
            UseTree::Path(UsePath {
                ident: segment.ident.clone(),
                colon2_token: Token![::](segment.span()),
                tree: Box::new(tree),
            })
        });
    let tree = UseTree::Path(UsePath {
        ident: Ident::new("super", source_span),
        colon2_token: Token![::](source_span),
        tree: Box::new(tree_body_and_tail),
    });
    let result = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Token![use](source_span),
        leading_colon: None,
        tree,
        semi_token: Token![;](source_span),
    };
    return result;
}
