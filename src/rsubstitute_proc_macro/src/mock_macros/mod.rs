pub(crate) mod ctx;
pub(crate) mod fn_decl;
pub(crate) mod fn_info_generation;
pub(crate) mod mock_generation;
pub(crate) mod mock_macro;
pub(crate) mod models;
pub(crate) mod struct_mock_syntax;
pub(crate) mod targets;

mod mocked_macro_mode;

pub use mocked_macro_mode::*;
