use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct MockStructTraitInfo {
    pub trait_path: Path,
    pub trait_ident_from_path: Ident,
    pub mock_type: MockType,
    pub fn_infos: Vec<FnInfo>,
    pub rest_impl_items: Vec<ImplItem>
}
