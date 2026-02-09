use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use proc_macro2::Ident;

pub struct MockStructTraitInfo {
    pub trait_ident_from_path: Ident,
    pub mock_type: MockType,
    pub fn_infos: Vec<FnInfo>,
}
