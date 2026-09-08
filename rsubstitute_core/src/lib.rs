#![allow(clippy::needless_return)]
#![allow(clippy::match_like_matches_macro)]
#![cfg_attr(feature = "debug_naming", allow(incomplete_features))]
#![cfg_attr(feature = "debug_naming", allow(unstable_features))]
#![cfg_attr(feature = "debug_naming", feature(specialization))]

pub mod args;
pub mod fn_parameters;
pub mod infrastructure;

pub mod settings;
mod lifetimes_transmutation;
pub mod times;

pub use infrastructure::verify_call_order;
