#![allow(clippy::needless_return)]
#![cfg_attr(feature = "debug_naming", allow(incomplete_features))]
#![cfg_attr(feature = "debug_naming", feature(specialization))]

pub mod args;
pub mod fn_parameters;
pub mod infrastructure;

mod config;
mod lifetimes_transmutation;
mod times;

pub use config::*;
pub use infrastructure::verify_call_order;
pub use times::*;
