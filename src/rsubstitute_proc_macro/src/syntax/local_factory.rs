use proc_macro2::Ident;
use syn::{Local, LocalInit, Pat, PatIdent};

pub trait ILocalFactory {
    fn create(&self, variable_ident: Ident, init: LocalInit) -> Local;
}

pub struct LocalFactory;

impl ILocalFactory for LocalFactory {
    fn create(&self, variable_ident: Ident, init: LocalInit) -> Local {
        let local = Local {
            attrs: Vec::new(),
            let_token: Default::default(),
            pat: Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: variable_ident,
                subpat: None,
            }),
            init: Some(init),
            semi_token: Default::default(),
        };
        return local;
    }
}
