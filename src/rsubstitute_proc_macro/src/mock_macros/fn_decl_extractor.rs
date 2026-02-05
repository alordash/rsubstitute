use crate::mock_macros::models::FnDecl;
use syn::*;

pub trait IFnDeclExtractor {
    fn extract(&self, items: &[TraitItem]) -> Vec<FnDecl>;

    fn extract_struct_fns(&self, impl_item_fns: &[ImplItemFn]) -> Vec<FnDecl>;

    fn extract_fn(&self, item_fn: &ItemFn) -> FnDecl;
}

pub struct FnDeclExtractor;

impl IFnDeclExtractor for FnDeclExtractor {
    fn extract(&self, trait_items: &[TraitItem]) -> Vec<FnDecl> {
        let fn_decls = trait_items
            .into_iter()
            .flat_map(|x| self.try_map_trait_item_fn(x))
            .collect();
        return fn_decls;
    }

    fn extract_struct_fns(&self, impl_item_fns: &[ImplItemFn]) -> Vec<FnDecl> {
        let fn_decls = impl_item_fns
            .iter()
            .map(|x| self.map_impl_item_fn(x))
            .collect();
        return fn_decls;
    }

    fn extract_fn(&self, item_fn: &ItemFn) -> FnDecl {
        let fn_decl = FnDecl {
            ident: item_fn.sig.ident.clone(),
            arguments: item_fn.sig.inputs.iter().cloned().collect(),
            return_value: item_fn.sig.output.clone(),
            maybe_base_fn_block: Some(*item_fn.block.clone()),
        };
        return fn_decl;
    }
}

impl FnDeclExtractor {
    fn try_map_trait_item_fn(&self, trait_item: &TraitItem) -> Option<FnDecl> {
        match trait_item {
            TraitItem::Fn(trait_item_fn) => {
                self.validate_signature(&trait_item_fn.sig);
                Some(self.map_trait_item_fn(trait_item_fn))
            }
            _ => None,
        }
    }

    fn map_trait_item_fn(&self, trait_item_fn: &TraitItemFn) -> FnDecl {
        let sig = &trait_item_fn.sig;
        let fn_decl = FnDecl {
            ident: sig.ident.clone(),
            arguments: sig.inputs.iter().cloned().collect(),
            return_value: sig.output.clone(),
            maybe_base_fn_block: trait_item_fn.default.clone(),
        };
        return fn_decl;
    }

    fn map_impl_item_fn(&self, impl_item_fn: &ImplItemFn) -> FnDecl {
        let sig = &impl_item_fn.sig;
        self.validate_signature(sig);
        let fn_decl = FnDecl {
            ident: sig.ident.clone(),
            arguments: sig.inputs.iter().cloned().collect(),
            return_value: sig.output.clone(),
            maybe_base_fn_block: Some(impl_item_fn.block.clone()),
        };
        return fn_decl;
    }

    fn validate_signature(&self, sig: &Signature) {
        if !sig.generics.params.is_empty() {
            panic!("Generic type parameters for trait functions are not supported.");
        }
    }
}
