pub mod common {
    pub mod fn_handle_stmt;
}

pub mod models {
    mod base_fn_kind;
    mod static_fn_mock_struct;
    mod trait_mock_struct;

    pub use base_fn_kind::*;
    pub use static_fn_mock_struct::*;
    pub use trait_mock_struct::*;
}

pub mod associated_fn_block;
pub mod static_fn_block;
pub mod static_fn_mock_struct;
pub mod trait_mock_struct;
