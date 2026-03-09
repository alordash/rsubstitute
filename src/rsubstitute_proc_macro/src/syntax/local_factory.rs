use proc_macro2::Ident;
use syn::*;

pub trait ILocalFactory {
    fn create(&self, variable_ident: Ident, init: LocalInit) -> Local;

    fn create_with_type(
        &self,
        variable_ident: Ident,
        variable_type: Type,
        init: LocalInit,
    ) -> Local;
}

pub struct LocalFactory;

impl ILocalFactory for LocalFactory {
    fn create(&self, variable_ident: Ident, init: LocalInit) -> Local {
        let local = Local {
            attrs: Vec::new(),
            let_token: Default::default(),
            pat: self.create_pat_ident(variable_ident),
            init: Some(init),
            semi_token: Default::default(),
        };
        return local;
    }

    fn create_with_type(
        &self,
        variable_ident: Ident,
        variable_type: Type,
        init: LocalInit,
    ) -> Local {
        let local = Local {
            attrs: Vec::new(),
            let_token: Default::default(),
            pat: Pat::Type(PatType {
                attrs: Vec::new(),
                pat: Box::new(self.create_pat_ident(variable_ident)),
                colon_token: Default::default(),
                ty: Box::new(variable_type),
            }),
            init: Some(init),
            semi_token: Default::default(),
        };
        return local;
    }
}

impl LocalFactory {
    fn create_pat_ident(&self, variable_ident: Ident) -> Pat {
        Pat::Ident(PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            mutability: None,
            ident: variable_ident,
            subpat: None,
        })
    }
}
