pub mod models {
    mod static_fn_mock_struct;
    mod trait_mock_struct;

    pub use static_fn_mock_struct::*;
    pub use trait_mock_struct::*;
}

pub mod static_fn_block;
pub mod static_fn_mock_struct;
pub mod trait_mock_struct;
