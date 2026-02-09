use crate::mock_macros::mock_generation::models::*;
use crate::syntax::IFieldRequiredIdentGetter;

pub trait IInnerDataParamGenerator {
    fn generate<'a>(&'a self, inner_data_struct: &'a InnerDataStruct) -> InnerDataParam<'a>;
}

pub(crate) struct InnerDataParamGenerator;

impl IInnerDataParamGenerator for InnerDataParamGenerator {
    fn generate<'a>(&'a self, inner_data_struct: &'a InnerDataStruct) -> InnerDataParam<'a> {
        let inner_data_param = InnerDataParam {
            inner_data_struct: &inner_data_struct,
            constructor_arguments: inner_data_struct
                .item_struct
                .fields
                .iter()
                .map(|field| (field.get_required_ident().clone(), field.ty.clone()))
                .collect(),
        };
        return inner_data_param;
    }
}
