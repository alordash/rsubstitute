use syn::*;

pub(crate) fn new(mod_ident: Ident, target_ident: Ident) -> ItemUse {
    let result = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](mod_ident.span())),
        use_token: Token![use](mod_ident.span()),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: mod_ident.clone(),
            colon2_token: Token![::](mod_ident.span()),
            tree: Box::new(UseTree::Name(UseName {
                ident: target_ident,
            })),
        }),
        semi_token: Token![;](mod_ident.span()),
    };
    return result;
}
