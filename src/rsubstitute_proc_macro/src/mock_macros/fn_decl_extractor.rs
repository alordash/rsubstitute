use crate::mock_macros::models::*;
use quote::format_ident;
use syn::*;

pub trait IFnDeclExtractor {
    fn extract(&self, items: &[TraitItem]) -> Vec<FnDecl>;

    fn extract_struct_fns(&self, impl_item_fns: &[&ImplItemFn]) -> Vec<FnDecl>;

    fn extract_struct_trait_impl_fns(&self, trait_impl: &TraitImpl) -> Vec<FnDecl>;

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

    fn extract_struct_fns(&self, impl_item_fns: &[&ImplItemFn]) -> Vec<FnDecl> {
        let fn_decls = impl_item_fns
            .iter()
            .map(|x| self.map_impl_item_fn(x))
            .collect();
        return fn_decls;
    }

    fn extract_struct_trait_impl_fns(&self, trait_impl: &TraitImpl) -> Vec<FnDecl> {
        let trait_ident = trait_impl.get_trait_ident_from_path();
        let fn_decls = trait_impl
            .get_fns()
            .iter()
            .map(move |trait_impl_fn| {
                self.create_fn_decl(
                    &trait_impl_fn.sig,
                    Some(trait_impl_fn.block.clone()),
                    Some(trait_ident.clone()),
                )
            })
            .collect();
        return fn_decls;
    }

    fn extract_fn(&self, item_fn: &ItemFn) -> FnDecl {
        let fn_decl = self.create_fn_decl(&item_fn.sig, Some(*item_fn.block.clone()), None);
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
        let fn_decl = self.create_fn_decl(sig, trait_item_fn.default.clone(), None);
        return fn_decl;
    }

    fn map_impl_item_fn(&self, impl_item_fn: &ImplItemFn) -> FnDecl {
        let sig = &impl_item_fn.sig;
        self.validate_signature(sig);
        let fn_decl = self.create_fn_decl(sig, Some(impl_item_fn.block.clone()), None);
        return fn_decl;
    }

    fn validate_signature(&self, sig: &Signature) {
        if !sig.generics.params.is_empty() {
            panic!("Generic type parameters for trait functions are not supported.");
        }
    }

    fn create_fn_decl(
        &self,
        sig: &Signature,
        maybe_base_fn_block: Option<Block>,
        maybe_parent_trait_ident: Option<Ident>,
    ) -> FnDecl {
        let ident = if let Some(parent_trait_ident) = maybe_parent_trait_ident {
            let original_ident = &sig.ident;
            format_ident!("{parent_trait_ident}_{original_ident}")
        } else {
            sig.ident.clone()
        };
        let fn_decl = FnDecl {
            ident,
            arguments: sig.inputs.iter().cloned().collect(),
            return_value: sig.output.clone(),
            maybe_base_fn_block,
            raw_fn_ident: sig.ident.clone(),
        };
        return fn_decl;
    }

    fn generate_ident_from_parent_trait_path(&self, original_ident: &Ident, path: &Path) -> Ident {
        let parent_trait_path_idents: Vec<_> =
            path.segments.iter().map(|x| x.ident.to_string()).collect();
        let joined_parent_trait_path_idents = parent_trait_path_idents.join("_");
        return format_ident!("{joined_parent_trait_path_idents}_{original_ident}");
    }
}
