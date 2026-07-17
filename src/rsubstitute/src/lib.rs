#![allow(clippy::needless_return)]
pub use rsubstitute_core::args::*;
pub use rsubstitute_core::*;

pub use rsubstitute_core::infrastructure::Mockable;
pub use rsubstitute_proc_macro::*;

// TODO - review it, maybe something is no longer needed
#[allow(unused_imports)]
pub mod for_generated {
    pub use crate::*;
    pub use rsubstitute_core::fn_parameters::*;
    pub use rsubstitute_core::infrastructure::*;
    pub use rsubstitute_core::*;
    pub use std::cell::LazyCell;
    pub use std::cell::RefCell;
    pub use std::fmt::Debug;
    pub use std::hash::Hash;
    pub use std::marker::PhantomData;
    pub use std::ops::Deref;
    pub use std::sync::Arc;
    pub use std::sync::LazyLock;
}
