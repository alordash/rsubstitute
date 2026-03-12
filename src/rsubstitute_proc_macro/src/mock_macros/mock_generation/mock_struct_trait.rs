use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;

pub(crate) fn generate(
    data_struct: &MockDataStruct,
    mock_struct_trait_info: MockStructTraitInfo,
) -> MockStructTrait {
    let setup_struct = mock_setup_struct::generate(
        &mock_struct_trait_info.trait_ident_from_path,
        &mock_struct_trait_info.mock_type,
        data_struct,
        Vec::new(),
    );
    let received_struct = mock_received_struct::generate(
        &mock_struct_trait_info.trait_ident_from_path,
        &mock_struct_trait_info.mock_type,
        data_struct,
        Vec::new(),
    );
    let setup_impl = mock_setup_impl::generate_for_trait(
        &mock_struct_trait_info.mock_type,
        &setup_struct,
        &mock_struct_trait_info.fn_infos,
    );
    let received_impl = mock_received_impl::generate_for_struct_trait(
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
