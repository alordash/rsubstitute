use crate::mock_macros::mock_generation::models::BaseFn;
use quote::format_ident;
use syn::ItemFn;

pub trait IBaseFnGenerator {
    fn generate(&self, item_fn: ItemFn) -> BaseFn;
}

pub(crate) struct BaseFnGenerator;

impl IBaseFnGenerator for BaseFnGenerator {
    fn generate(&self, mut item_fn: ItemFn) -> BaseFn {
        let new_ident = format_ident!("{}_{}", Self::IDENT_PREFIX, item_fn.sig.ident);
        item_fn.sig.ident = new_ident;
        let base_fn = BaseFn { item_fn };
        return base_fn;
    }
}

impl BaseFnGenerator {
    const IDENT_PREFIX: &'static str = "base";
}
