pub use rsubstitute_core::args::*;
pub use rsubstitute_core::*;

mod global_mocks_map;

pub mod macros {
    pub use rsubstitute_proc_macro::*;
}

#[allow(unused_imports)]
pub mod for_generated {
    pub use crate::*;
    pub use global_mocks_map::*;
    pub use macros::*;
    pub use rsubstitute_core::mock_data::*;
    pub use rsubstitute_core::*;
    pub use std::cell::LazyCell;
    pub use std::cell::RefCell;
    pub use std::hash::Hash;
    pub use std::marker::PhantomData;
    pub use std::ops::Deref;
    pub use std::sync::Arc;
    pub use std::sync::LazyLock;
    pub use std::fmt::Debug;
}
