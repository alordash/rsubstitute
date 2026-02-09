use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use std::sync::Arc;

pub trait IMockStructTraitGenerator {
    fn generate(
        &self,
        data_struct: &MockDataStruct,
        mock_struct_trait_info: MockStructTraitInfo,
    ) -> MockStructTrait;
}

pub(crate) struct MockStructTraitGenerator {
    pub mock_setup_struct_generator: Arc<dyn IMockSetupStructGenerator>,
    pub mock_received_struct_generator: Arc<dyn IMockReceivedStructGenerator>,
    pub mock_setup_impl_generator: Arc<dyn IMockSetupImplGenerator>,
    pub mock_received_impl_generator: Arc<dyn IMockReceivedImplGenerator>,
}

impl IMockStructTraitGenerator for MockStructTraitGenerator {
    fn generate(
        &self,
        data_struct: &MockDataStruct,
        mock_struct_trait_info: MockStructTraitInfo,
    ) -> MockStructTrait {
        let setup_struct = self.mock_setup_struct_generator.generate(
            &mock_struct_trait_info.trait_ident_from_path,
            &mock_struct_trait_info.mock_type,
            data_struct,
            Vec::new()
        );
        let received_struct = self.mock_received_struct_generator.generate(
            &mock_struct_trait_info.trait_ident_from_path,
            &mock_struct_trait_info.mock_type,
            data_struct,
            Vec::new()
        );
        let setup_impl = self.mock_setup_impl_generator.generate_for_trait(
            &mock_struct_trait_info.mock_type,
            &setup_struct,
            &mock_struct_trait_info.fn_infos,
        );
        let received_impl = self.mock_received_impl_generator.generate_for_trait(
            &mock_struct_trait_info.mock_type,
            &received_struct,
            &mock_struct_trait_info.fn_infos,
        );
        let mock_struct_trait = MockStructTrait {
            info: mock_struct_trait_info,
            setup_struct,
            received_struct,
            setup_impl,
            received_impl,
        };
        return mock_struct_trait;
    }
}
