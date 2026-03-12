use proc_macro2::Ident;
use syn::*;

pub(crate) fn create(variable_ident: Ident, init: LocalInit) -> Local {
    let local = Local {
        attrs: Vec::new(),
        let_token: Default::default(),
        pat: create_pat_ident(variable_ident),
        init: Some(init),
        semi_token: Default::default(),
    };
    return local;
}

pub(crate) fn create_with_type(
    variable_ident: Ident,
    variable_type: Type,
    init: LocalInit,
) -> Local {
    let local = Local {
        attrs: Vec::new(),
        let_token: Default::default(),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(create_pat_ident(variable_ident)),
            colon_token: Default::default(),
            ty: Box::new(variable_type),
        }),
        init: Some(init),
        semi_token: Default::default(),
    };
    return local;
}

fn create_pat_ident(variable_ident: Ident) -> Pat {
    Pat::Ident(PatIdent {
        attrs: Vec::new(),
        by_ref: None,
        mutability: None,
        ident: variable_ident,
        subpat: None,
    })
}
