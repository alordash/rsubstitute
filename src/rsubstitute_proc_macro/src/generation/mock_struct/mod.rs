mod common {
    pub mod control_creation_fn;
    pub mod fn_handle_stmt;
    pub mod mock_struct_fn_new;
}

pub mod models {
    mod associated_controls;
    mod base_fn_kind;
    mod static_controls;
    mod static_fn_mock_struct;
    mod struct_mock_struct;
    mod trait_mock_struct;

    pub(crate) use associated_controls::*;
    pub(crate) use base_fn_kind::*;
    pub(crate) use static_controls::*;
    pub(crate) use static_fn_mock_struct::*;
    pub(crate) use struct_mock_struct::*;
    pub(crate) use trait_mock_struct::*;
}

pub mod associated_method_block;
pub mod static_fn_block;
pub mod static_fn_mock_struct;
pub mod struct_mock_struct;
pub mod trait_mock_struct;
