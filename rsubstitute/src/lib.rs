#![allow(clippy::needless_return)]

#![doc = include_str!("../../README.md")]
pub use rsubstitute_proc_macro::mock;

pub use rsubstitute_core::args::*;
pub use rsubstitute_core::verify_call_order;
pub use rsubstitute_core::*;

pub use rsubstitute_core::infrastructure::Mockable;

/// asd
/// ```rust
/// use rsubstitute::*;
/// 
/// // #[mock]
/// trait Trait {
///     fn work(&self, v: i32) -> i32;
/// }
/// 
/// fn use_trait(t: &dyn Trait, v: i32) -> i32 {
///     t.work(v)
/// }
/// 
/// // Arrange
/// let mut mock = TraitMock::new();
/// mock.setup().work(10).returns(20);
/// 
/// // Act
/// let result = use_trait(&mock, 10);
/// 
/// // Assert
/// assert_eq!(result, 20);
/// mock.received().work(10, 1.time());
/// ```
pub mod for_generated {
    pub use rsubstitute_core::args::*;
    pub use rsubstitute_core::fn_parameters::*;
    pub use rsubstitute_core::infrastructure::*;
    pub use rsubstitute_core::*;
}
