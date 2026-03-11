#![allow(clippy::needless_return)]
#![cfg_attr(feature = "debug_naming", allow(incomplete_features))]
#![cfg_attr(feature = "debug_naming", feature(specialization))]

pub mod args;
pub mod fn_parameters;
pub mod mock_data;

mod config;
mod di;
mod error_printer;
mod generics_hash_key;
mod lifetimes_transmutation;
mod matching_config_search_result;
mod times;

pub use config::*;
pub use generics_hash_key::*;
pub use times::*;

pub use crate::di::SERVICES;
