use crate::mock_macros::models::FnDecl;
use syn::*;

pub trait IFnDeclExtractor {
    fn extract(&self, items: &[TraitItem]) -> Vec<FnDecl>;

    fn extract_fn(&self, item_fn: ItemFn) -> FnDecl;
}

pub struct FnDeclExtractor;

impl IFnDeclExtractor for FnDeclExtractor {
    fn extract(&self, trait_items: &[TraitItem]) -> Vec<FnDecl> {
        let fn_decls = trait_items
            .into_iter()
            .flat_map(|x| self.try_map(x))
            .collect();
        return fn_decls;
    }

    fn extract_fn(&self, item_fn: ItemFn) -> FnDecl {
        let fn_decl = FnDecl {
            ident: item_fn.sig.ident.clone(),
            arguments: item_fn.sig.inputs.iter().cloned().collect(),
            return_value: item_fn.sig.output.clone(),
        };
        return fn_decl;
    }
}

impl FnDeclExtractor {
    fn try_map(&self, trait_item: &TraitItem) -> Option<FnDecl> {
        match trait_item {
            TraitItem::Fn(trait_item_fn) => Some(self.map(trait_item_fn)),
            _ => None,
        }
    }

    fn map(&self, trait_item_fn: &TraitItemFn) -> FnDecl {
        let sig = &trait_item_fn.sig;
        let fn_decl = FnDecl {
            ident: sig.ident.clone(),
            arguments: sig.inputs.iter().cloned().collect(),
            return_value: sig.output.clone(),
        };
        return fn_decl;
    }
}
