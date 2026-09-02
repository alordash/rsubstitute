//! Library for mocking Rust static functions, traits and structures designed to follow
//! arrange-act-assert pattern.
//!
//! # Usage
//! Just apply `#[mock]` attribute on your function, trait, structure or `impl` block. You can also
//! use `#[mock(base)]` if you want the ability to use base implementation in your tests (refer to
//! [`Base implementation`](#base-implementation) section in user guide).
//!
//! ```rust
//! use rsubstitute::*;
//! #[mock] trait Trait {}
//!
//! # fn main() {
//! let mock = TraitMock::new();
//! # }
//! ```
//!
//! Automatically generated mock structure has two special methods to control it's behaviour:
//! `setup()` and `received()`. `setup()` allows you to configure what mock object should do when
//! it's methods are called. `received()` is used to check how mock object was used.
//! ```rust
//! # use rsubstitute::*;
//!
//! #[mock]
//! trait Trait {
//!     fn work(&self, v: i32) -> i32;
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! mock.setup()
//!     .work(1).returns(10)    // when called as `work(1)` it will return 10
//!     .work(2).returns(20);   // when called as `work(2)` it will return 20
//!
//! // Act
//! let result = mock.work(1);
//!
//! // Assert
//! assert_eq!(result, 10);
//! mock.received()
//!     .work(1, 1.time())      // verify that `work(1)` was called once
//!     .work(2, Times::Never); // verify that `work(2)` was never called
//! # }
//! ```
//!
//!
//! # User guide
//!
//! * [`Trait mock`](#trait-mock)
//! * [`Structure mock`](#structure-mock)
//! * [`Trait implementation mock`](#trait-implementation-mock)
//!
//! ## Trait mock
//!
//! To mock trait add `#[mock]` or `#[mock(base)]` attribute to it.
//! ```
//! use rsubstitute::*;
//!
//! #[mock]
//! trait Trait {
//!     fn work(&self, v: i32) -> i32;
//! }
//!
//! fn use_trait(t: &dyn Trait, v: i32) -> i32 {
//!     t.work(v)
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! mock.setup()
//!     .work(1).returns(10)
//!     .work(2).returns(20);
//!
//! // Act
//! let first  = use_trait(&mock, 1);
//! let second = use_trait(&mock, 2);
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 20);
//! mock.received()
//!     .work(1, 1.time())
//!     .work(2, 1.time());
//! # }
//! ```
//! 
//! There is one limitation: `rsubstitute` can't mock trait that has super traits (other than
//! `Clone`). For example, this won't compile:
//! ```ignore
//! #[mock] trait Trait: SuperTrait {}
//! ```
//! But this will:
//! ```
//! # use rsubstitute::*;
//! #[mock] trait Trait: Clone {}
//! # fn main() {}
//! ```
//!
//! ## Structure mock
//!
//! `rsubstitute` supports mocking structures, but it's intended to be used only on structures that
//! behave more like simply "stateful" functions.  
//! To mock structure add `#[mock]` attribute to structure definition and `#[mock]` or
//! `#[mock(base)]` to it's `impl` blocks whose functionality you want to mock.
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock]
//! struct Structure { v: i32 }
//!
//! #[mock(base)]
//! impl Structure {
//!     pub fn new(v: i32) -> Self { Self { v } }
//! }
//!
//! #[mock]
//! impl Structure {
//!     pub fn get(&self) -> i32 { self.v }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = Structure::new(10);
//! mock.setup().get().returns(20);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.received().get(1.time());
//! # }
//! ```
//! <div class="warning">
//!
//! There are a couple of limitations for structures mocking:
//! 1. Mocked structure can not be constructed or deconstructed outside of associated functions that
//! were mocked. This is because `rsubstitute` adds special `__rs_data` field to generated structure
//! that it automatically fills inside mocked `impl` block.
//! 2. Structure must have either named fields or no fields at all. `struct Struct { v: i32 }` and
//! `struct Struct;` can be mocked, but `struct Struct(i32)` can not.
//! 3. Only functions inside mocked `impl` blocks can be mocked. In the example below only `foo` can
//! be mocked; `bar` will always use base implementation:
//! ```rust
//! # use rsubstitute::*;
//! #[mock] struct Structure;
//!
//! #[mock]
//! impl Structure {
//!     fn foo(&self) -> i32 { 1 } // mockable
//! }
//!
//! impl Structure {
//!     fn bar(&self) -> i32 { 2 } // unmockable - will always return 2
//! }
//!
//! # fn main() {}
//! ```
//! 4. To add `#[mock]` to structure's `impl` blocks the structure itself must have `#[mock]`
//! attribute.
//! 5. Can not mock functions separated by `#[cfg]`. This won't compile:
//! ```ignore
//! #[mock]
//! impl Structure {
//!     #[cfg(test)]      fn work(&self) {}
//!     #[cfg(not(test))] fn work(&self) {}
//! }
//! ```
//!
//! </div>
//!
//! ## Trait implementation mock
//! `rsubstitute` supports mocking implementations of traits on mockable structures (trait itself
//! does not need to be mockable) by adding `#[mock]` or `#[mock(base]` on `impl` block. Each mocked
//! trait implementation adds `as_TRAIT_NAME` method both to mock's `setup` and `received`
//! functions:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] struct Structure;
//! #[mock(base)]
//! impl Structure {
//!     pub fn new() -> Self { Self }
//! }
//!
//! trait Trait {
//!     fn get(&self) -> i32;
//! }
//!
//! #[mock]
//! impl Trait for Structure {
//!     fn get(&self) -> i32 { 10 }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = Structure::new();
//! mock.setup().as_Trait().get().returns(20);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.received().as_Trait().get(1.time());
//! # }
//! ```
//!
//! <div class="warning">
//!
//! There are a couple of limitations for trait implementations mocking:
//! 1. Can not mock more than one implementation of same trait on a struct differing only in trait's
//! generics. For example, this won't compile:
//! ```ignore
//! # use rsubstitute::*;
//! #[mock]
//! impl From<i32> for Struct {
//!     //...
//! }
//! #[mock]
//! impl From<usize> for Struct {
//!     //...
//! }
//! ```
//! Can mock only one of them:
//! ```rust
//! # use rsubstitute::*;
//! # #[mock] struct Struct;
//! #[mock]
//! impl From<i32> for Struct {
//! #    fn from(value: i32) -> Self { Self }
//!     //...
//! }
//! impl From<usize> for Struct {
//! #    fn from(value: usize) -> Self { Self { __rs_data: Default::default() } }
//!     //...
//! }
//!
//! # fn main() {}
//! ```
//! 2. If trait has default implementations for some methods, these methods can be mocked only if
//! they are defined inside structure's `impl` block. For example:
//! ```ignore
//! trait Trait {
//!     fn get(&self) -> i32 { 10 }
//! }
//! 
//! // Can't mock `Trait::get()`
//! impl Trait for Struct {}
//! 
//! // Can mock `Trait::get()`
//! impl Trait for Struct {
//!     fn get(&self) -> i32 { 10 }
//! }
//! ```
//! 3. Limitations from [`Structure mock`](#structure-mock).
//!
//! </div>
#![allow(clippy::needless_return)]
pub use rsubstitute_proc_macro::mock;

/// TODO - append to outer doc comment
/// # How it works
///
/// Easiest way to mock some function is to create two separate versions of it - one for `release`
/// build and one for `test` that tracks calls:
/// ```rust
/// #[cfg(not(test))] fn f() {}
///
/// #[cfg(test)] fn f() { F_CALLS_COUNT += 1; }
/// #[cfg(test)] static mut F_CALLS_COUNT: usize = 0;
///
/// fn payload() { f() }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///     #[test]
///     fn payload_test() {
///         // Act
///         payload();
///         // Assert
///         assert_eq!(F_CALLS_COUNT, 1);
///     }
/// }
/// ```
/// This is basically what `rsubstitute` does - it automatically creates infrastructure for mocking,
/// except that it generates a more complex code for flexible configuration.
pub use rsubstitute_core::args::*;
pub use rsubstitute_core::verify_call_order;
pub use rsubstitute_core::*;

pub use rsubstitute_core::infrastructure::Mockable;

pub mod for_generated {
    pub use rsubstitute_core::args::*;
    pub use rsubstitute_core::fn_parameters::*;
    pub use rsubstitute_core::infrastructure::*;
    pub use rsubstitute_core::*;
}
