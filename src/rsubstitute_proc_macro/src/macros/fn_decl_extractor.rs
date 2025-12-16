use crate::macros::models::FnDecl;
use syn::{ReturnType, TraitItem, TraitItemFn};

pub trait IFnDeclExtractor {
    fn extract(&self, items: Vec<TraitItem>) -> Vec<FnDecl>;
}

pub struct FnDeclExtractor;

impl IFnDeclExtractor for FnDeclExtractor {
    fn extract(&self, trait_items: Vec<TraitItem>) -> Vec<FnDecl> {
        let fn_decls = trait_items
            .into_iter()
            .flat_map(|x| self.try_map(x))
            .collect();
        return fn_decls;
    }
}

impl FnDeclExtractor {
    fn try_map(&self, trait_item: TraitItem) -> Option<FnDecl> {
        match trait_item {
            TraitItem::Fn(trait_item_fn) => Some(self.map(trait_item_fn)),
            _ => None,
        }
    }

    fn map(&self, trait_item_fn: TraitItemFn) -> FnDecl {
        let sig = trait_item_fn.sig;
        let fn_decl = FnDecl {
            ident: sig.ident,
            arguments: sig.inputs.into_iter().collect(),
            return_value: sig.output
        };
        return fn_decl;
    }
}
