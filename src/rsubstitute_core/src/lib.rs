#![cfg_attr(feature = "debug_naming", feature(specialization))]

pub mod args;

mod config;
mod di;
mod error_printer;
mod fn_config;
mod fn_data;
mod fn_parameters;
mod i_base_caller;
mod i_mock_data;
mod i_mut_ref_clone;
mod matching_config_search_result;
mod shared_fn_config;
mod static_local_key;
mod times;

pub use config::*;
pub use fn_config::*;
pub use fn_data::*;
pub use fn_parameters::*;
pub use i_base_caller::*;
pub use i_mock_data::*;
pub use shared_fn_config::*;
pub use static_local_key::*;
pub use times::*;

pub use crate::di::SERVICES;
