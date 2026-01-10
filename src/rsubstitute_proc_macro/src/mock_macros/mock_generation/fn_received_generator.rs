use syn::*;
use crate::mock_macros::fn_info_generation::models::*;

pub trait IFnReceivedGenerator {
    fn generate(&self, fn_info: &FnInfo) -> ItemFn;
}

pub(crate) struct FnReceivedGenerator;

impl IFnReceivedGenerator for FnReceivedGenerator {
    fn generate(&self, fn_info: &FnInfo) -> ItemFn {
        todo!()
    }
}