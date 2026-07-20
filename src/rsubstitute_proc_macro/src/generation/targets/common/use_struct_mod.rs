use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use syn::*;

pub(crate) fn generate(source_span: Span, target_path: &Path) -> ItemUse {
    let result = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Token![use](source_span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: Ident::new("super", source_span),
            colon2_token: Token![::](source_span),
            tree: Box::new(UseTree::Path(UsePath {
                ident: format_ident!(
                    "__rsubstitute_generated_{}Mock",
                    path::last_ident(target_path)
                ),
                colon2_token: Token![::](source_span),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Token![*](source_span),
                })),
            })),
        }),
        semi_token: Token![;](source_span),
    };
    result
}
