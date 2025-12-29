pub use rsubstitute_core::args_matching::*;
pub use rsubstitute_core::*;

pub mod macros {
    pub use rsubstitute_proc_macro::*;
}

pub mod for_generated {
    pub use crate::*;
    pub use macros::*;
    pub use std::cell::LazyCell;
    pub use std::marker::PhantomData;
    pub use std::sync::Arc;
    pub use std::sync::LazyLock;
}

// TODO - move to separate crate?
pub mod assertions;
