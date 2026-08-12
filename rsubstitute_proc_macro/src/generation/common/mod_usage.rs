use crate::syntax::attributes;
use syn::*;

pub(crate) fn new<const N: usize>(mod_ident: Ident, target_idents: [Ident; N]) -> ItemUse {
    let span = mod_ident.span();
    let result = ItemUse {
        attrs: vec![attributes::allow_unreachable_pub(span)],
        vis: Visibility::Public(Token![pub](span)),
        use_token: Token![use](span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: mod_ident.clone(),
            colon2_token: Token![::](mod_ident.span()),
            tree: Box::new(UseTree::Group(UseGroup {
                brace_token: token::Brace(span),
                items: target_idents
                    .into_iter()
                    .map(|ident| UseTree::Name(UseName { ident }))
                    .collect(),
            })),
        }),
        semi_token: Token![;](span),
    };
    return result;
}

pub(crate) fn new_all(mod_ident: Ident) -> ItemUse {
    new_core(mod_ident, false)
}

pub(crate) fn new_pub_all(mod_ident: Ident) -> ItemUse {
    new_core(mod_ident, true)
}

#[inline]
fn new_core(mod_ident: Ident, public: bool) -> ItemUse {
    let span = mod_ident.span();
    let result = ItemUse {
        attrs: vec![attributes::allow_unreachable_pub(span)],
        vis: if public {
            Visibility::Public(Token![pub](span))
        } else {
            Visibility::Inherited
        },
        use_token: Token![use](span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: mod_ident.clone(),
            colon2_token: Token![::](mod_ident.span()),
            tree: Box::new(UseTree::Glob(UseGlob {
                star_token: Token![*](span),
            })),
        }),
        semi_token: Token![;](span),
    };
    return result;
}
