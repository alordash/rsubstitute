use crate::macros::models::TargetDecl;
use syn::ItemTrait;

pub trait ITargetDeclExtractor {
    fn extract(&self, item_trait: &ItemTrait) -> TargetDecl;
}

pub struct TargetDeclExtractor;

impl ITargetDeclExtractor for TargetDeclExtractor {
    fn extract(&self, item_trait: &ItemTrait) -> TargetDecl {
        let target_decl = TargetDecl {
            ident: item_trait.ident.clone(),
        };
        return target_decl;
    }
}
