use crate::constants;
use crate::mock_generation::clone_for_rsubstitute_trait_impl;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(source_struct: ItemStruct) -> InnerDataStruct {
    let mut item_struct = source_struct.clone();
    item_struct
        .attrs
        .push(constants::DOC_HIDDEN_ATTRIBUTE.clone());
    item_struct.ident = format_ident!("{}_{}", item_struct.ident, INNER_DATA_STRUCT_IDENT_SUFFIX);
    item_struct.vis = Visibility::Public(Default::default());
    let ty = r#type::create_from_struct(&item_struct);
    let clone_for_rsubstitute_trait_impl = clone_for_rsubstitute_trait_impl::generate(&item_struct);
    let inner_data_struct = InnerDataStruct {
        item_struct,
        ty,
        clone_for_rsubstitute_trait_impl,
    };
    return inner_data_struct;
}

const INNER_DATA_STRUCT_IDENT_SUFFIX: &'static str = "InnerData";
