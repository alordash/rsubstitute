pub use rsubstitute_core::args_matching::*;
pub use rsubstitute_core::*;

pub mod macros {
    pub use rsubstitute_proc_macro::*;
}

pub mod prelude {
    pub use crate::*;
    pub use macros::*;
}
