use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use syn::*;

pub trait IBaseCallerImplGenerator {
    fn generate(&self, mock_type: &MockType, base_impl_fn_block: Block) -> BaseCallerImpl;
}

pub(crate) struct BaseCallerImplGenerator;

impl IBaseCallerImplGenerator for BaseCallerImplGenerator {
    fn generate(&self, mock_type: &MockType, base_impl_fn_block: Block) -> BaseCallerImpl {}
}
