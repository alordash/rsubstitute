use crate::macros::models::FnInfo;
use syn::{ReturnType, TraitItem, TraitItemFn};

pub trait IFnInfoExtractor {
    fn extract(&self, items: Vec<TraitItem>) -> Vec<FnInfo>;
}

pub struct FnInfoExtractor;

impl IFnInfoExtractor for FnInfoExtractor {
    fn extract(&self, trait_items: Vec<TraitItem>) -> Vec<FnInfo> {
        let fn_infos = trait_items
            .into_iter()
            .flat_map(|x| self.try_map(x))
            .collect();
        return fn_infos;
    }
}

impl FnInfoExtractor {
    fn try_map(&self, trait_item: TraitItem) -> Option<FnInfo> {
        match trait_item {
            TraitItem::Fn(trait_item_fn) => Some(self.map(trait_item_fn)),
            _ => None,
        }
    }

    fn map(&self, trait_item_fn: TraitItemFn) -> FnInfo {
        let sig = trait_item_fn.sig;
        let fn_info = FnInfo {
            ident: sig.ident,
            arguments: sig.inputs.into_iter().collect(),
            return_value: match sig.output {
                ReturnType::Default => None,
                ReturnType::Type(_, r#type) => Some(r#type),
            },
        };
        return fn_info;
    }
}
