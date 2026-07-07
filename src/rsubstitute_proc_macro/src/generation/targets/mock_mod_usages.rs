use crate::generation::targets::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span) -> MockModUsages {
    // TODO - need to remove it, everything from rsubstitute must be refered as `::rsubstitute::for_generated::*`
    // use rsubstitute_punctuated() like in data_field
    let use_rsubstitute_for_generated = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Token![use](span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: Ident::new("rsubstitute", span),
            colon2_token: Token![::](span),
            tree: Box::new(UseTree::Path(UsePath {
                ident: Ident::new("for_generated", span),
                colon2_token: Token![::](span),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Token![*](span),
                })),
            })),
        }),
        semi_token: Token![;](span),
    };
    let use_super = ItemUse {
        attrs: vec![attributes::allow_unused_imports(span)],
        vis: Visibility::Inherited,
        use_token: Token![use](span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: Ident::new("super", span),
            colon2_token: Token![::](span),
            tree: Box::new(UseTree::Glob(UseGlob {
                star_token: Token![*](span),
            })),
        }),
        semi_token: Token![;](span),
    };

    let result = MockModUsages {
        use_rsubstitute_for_generated,
        use_super,
    };
    return result;
}
