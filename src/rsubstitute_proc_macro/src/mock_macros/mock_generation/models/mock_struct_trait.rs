use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;

pub(crate) struct MockStructTrait {
    pub info: MockStructTraitInfo,
    pub setup_struct: MockSetupStruct,
    pub received_struct: MockReceivedStruct,
    pub setup_impl: MockSetupImpl,
    pub received_impl: MockReceivedImpl,
}