#![allow(clippy::needless_return)]
pub use rsubstitute_core::args::*;
pub use rsubstitute_core::verify_call_order;
pub use rsubstitute_core::*;

pub use rsubstitute_core::infrastructure::Mockable;
pub use rsubstitute_proc_macro::*;

pub mod for_generated {
    pub use rsubstitute_core::args::*;
    pub use rsubstitute_core::fn_parameters::*;
    pub use rsubstitute_core::infrastructure::*;
    pub use rsubstitute_core::*;
}
