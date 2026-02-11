use crate::mock_macros::mock_generation::models::*;

pub struct MockStructTrait {
    pub info: MockStructTraitInfo,
    pub setup_struct: MockSetupStruct,
    pub received_struct: MockReceivedStruct,
    pub setup_impl: MockSetupImpl,
    pub received_impl: MockReceivedImpl,
}