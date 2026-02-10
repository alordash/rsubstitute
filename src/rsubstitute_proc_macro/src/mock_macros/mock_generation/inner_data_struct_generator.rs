use crate::mock_macros::mock_generation::models::*;
use quote::format_ident;
use syn::*;

pub trait IInnerDataStructGenerator {
    fn generate(&self, source_struct: ItemStruct) -> InnerDataStruct;
}

pub(crate) struct InnerDataStructGenerator;

impl IInnerDataStructGenerator for InnerDataStructGenerator {
    fn generate(&self, source_struct: ItemStruct) -> InnerDataStruct {
        let mut item_struct = source_struct.clone();
        item_struct.ident = format_ident!(
            "{}_{}",
            item_struct.ident,
            Self::INNER_DATA_STRUCT_IDENT_SUFFIX
        );
        item_struct.vis = Visibility::Public(Default::default());
        let inner_data_struct = InnerDataStruct { item_struct };
        return inner_data_struct;
    }
}

impl InnerDataStructGenerator {
    const INNER_DATA_STRUCT_IDENT_SUFFIX: &'static str = "InnerData";
}
