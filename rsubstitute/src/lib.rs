//! Library for mocking Rust static functions, traits and structures.
//!
//! # Usage
//! Just apply `#[mock]` attribute on your function, trait, structure or `impl` block. You can also
//! use `#[mock(base)]` if you want the ability to use base implementation in your tests (refer to
//! [`Base implementation`](#base-implementation) for more information).
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
//! Automatically generated mock structure has two special methods to control its behavior:
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
//! let mut mock = TraitMock::new();    // `mock` must be mutable in order to be configured
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
//! # User guide
//!
//! * [Mocking traits](#mocking-traits)
//! * [Mocking structures](#mocking-structures)
//! * [Mocking trait implementations](#mocking-trait-implementations)
//! * [Mocking static functions](#mocking-static-functions)
//! * [Mocking static associated functions](#mocking-static-associated-functions)
//! * [Arguments matching](#arguments-matching)
//! * [Controlling function behavior](#controlling-function-behavior)
//! * [Base implementation](#using-base-implementation)
//! * [Verifying calls](#verifying-calls)
//! * [Generics](#generics)
//! * [Associated constants and types](#associated-constants-and-types)
//! * [`impl Trait` types](#impl-trait-types)
//! * [Trait modifiers](#trait-modifiers)
//! * [Function modifiers](#function-modifiers)
//! * [Call order validation](#call-order-validation)
//! * [Receiver types](#receiver-types)
//! * [Cloning mocks](#mocks-cloning)
//! * [Undefined behavior](#undefined-behavior) TODO
//! * [Limitations](#limitations)               TODO
//!
//! ## Mocking traits
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
//! ## Mocking structures
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
//! ## Mocking trait implementations
//! To mock implementations of traits on mockable structures (trait itself does not need to be
//! mockable) add `#[mock]` or `#[mock(base)]` on `impl` block. Each mocked trait implementation adds
//! `as_TRAIT_NAME` method both to mock's `setup` and `received` functions:
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
//! 3. Limitations from [`Mocking structures`](#mocking-structures).
//!
//! </div>
//!
//! ## Mocking static functions
//!
//! To mock static function add `#[mock]` or `#[mock(base)]` attribute. This will generate module
//! with the same name as mocked function, which exposes standalone `setup()` and `received()`
//! functions:
//!
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock]
//! fn work(v: i32) -> i32 { v + 1 }
//!
//! # fn main() {
//! // Arrange
//! work::setup(1).returns(10)
//!      .setup(2).returns(20);
//!
//! // Act
//! let first  = work(1);
//! let second = work(2);
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 20);
//! work::received(1, 1.time())
//!      .received(2, 1.time());
//! # }
//! ```
//!
//! There are a couple of limitations for functions mocking:
//! 1. Configuration for standalone function mock is stored in thread-local storage to prevent race
//! condition when running multiple tests in parallel. This may impact tests running on
//! work-stealing async runtimes.
//! 2. Calling `setup()` of standalone function clears it's all previous configurations to prevent
//! configuration from one test leaking into next sequentially ran test. Standalone function set-up
//! must happen in **single module-level `setup()` call** in each unit-test.  
//! For example, this is a wrong way of configuring standalone function:
//! ```ignore
//! #[mock] fn work(v: i32) -> i32 { v }
//!
//! work::setup(1).returns(10);
//! work::setup(2).returns(20); // `work::setup(2)` will clear previous configuration
//! ```
//! Here is a correct way: call `work::setup()` only once and then use chain of `.setup()` calls:
//! ```ignore
//! work::setup(1).returns(10)
//!      .setup(2).returns(20); // `.setup(2)` does not clear previous configuration
//! ```
//!
//! ## Mocking static associated functions
//!
//! Static associated functions are mocked almost the same way as regular static functions, except
//! that instead of `setup()` and `received()` you must use `static_setup()` and
//! `static_received()`. Here's example of mocking trait with static function:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock]
//! trait Trait {
//!     fn work(v: i32) -> i32;
//! }
//!
//! fn use_trait_impl<T: Trait>(v: i32)  -> i32 { T::work(v) }
//!
//! # fn main() {
//! // Arrange
//! TraitMock::static_setup().work(10).returns(20);
//!
//! // Act
//! let result = use_trait_impl::<TraitMock>(10);
//!
//! // Assert
//! assert_eq!(result, 20);
//! TraitMock::static_received().work(10, 1.time());
//! # }
//! ```
//!
//! Static functions in structure implementations can also be mocked:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] struct Struct;
//!
//! #[mock]
//! impl Struct {
//!     fn work(v: i32) -> i32 { v + 1 }
//! }
//!
//! # fn main() {
//! // Arrange
//! Struct::static_setup().work(10).returns(20);
//!
//! // Act
//! let result = Struct::work(10);
//!
//! // Assert
//! assert_eq!(result, 20);
//! Struct::static_received().work(10, 1.time());
//! # }
//! ```
//!
//! There are a couple of limitations:
//! 1. Associated functions that use base implementation using `#[mock(base)]` use base
//! implementation by default, without any configuration. This is done to make creation of structure
//! mocks simpler by just calling `Struct::new()` without needing to first do
//! `Struct::static_setup().new(Arg::Any, ...).call_base()` in each test.
//! 2. Limitations from [`Mocking static functions`](#mocking-static-functions)
//!
//! ## Arguments matching
//!
//! `setup()` and `received()` functions accept [`Arg<T>`] as arguments, where `T` is type of
//! argument in source function. `Arg` provides multiple ways to match argument's value:
//!
//! 1. [`Arg::eq`] - checks that argument is equal to provided value. Uses [`PartialEq::eq`] of `T`.
//! Can be used either manually like `mock.setup(Arg::eq(10))` or implicitly using `Into` conversion
//! like `mock.setup(10)`.
//! 2. [`Arg::is`] - checks that argument passes provided predicate. Usage example:
//! `mock.setup(Arg::is(|v| *v == 10))`
//! 3. [`Arg::not_eq`] - checks that argument is NOT equal to provided value. Uses [`Partial::eq`]
//! of `T`. Opposite of `Arg::eq`. Usage example: `mock.setup(Arg::not_eq(10))`.
//! 4. [`Arg::ref_eq`] - checks that argument's reference points to the same place as provided
//! reference. Compares referenes returned by [`std::ops::Deref::deref`] of `T`. Usage example:
//! ```rust
//! # use rsubstitute::*;
//! # use std::rc::Rc;
//! #[mock]
//! trait Trait {
//!     fn work(&self, r: Rc<i32>) -> i32;
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! let r1 = Rc::new(1);
//! let r2 = r1.clone();
//! mock.setup().work(Arg::ref_eq(r1.clone())).always_returns(10);
//!
//! // Act
//! let first  = mock.work(r1.clone());
//! let second = mock.work(r2);
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 10);
//! mock.received().work(Arg::ref_eq(r1), 2.times());
//! # }
//! ```
//! 5. [`Arg::ref_not_eq`] - checks that argument's reference DOES NOT point to the same place as
//! provided reference. Compares references returned by [`std::ops::Deref::deref`] of `T`. Opposite
//! of `Arg::ref_eq`.
//!
//! ## Controlling function behavior
//!
//! Calling `setup()` returns [`FnConfigurator`] - type that is used to tell mocked
//! function what it should do upon receiving matching call.
//!
//! ### Return values
//! Functions that have return values can set them in multiple ways:
//! 1. [`FnConfigurator::returns`] - sets single-use return value.
//! 2. [`FnConfigurator::returns_many`] - sets multiple single-use return values in
//! one call.
//! 3. [`FnConfigurator::always_returns`] - sets return value that will can be
//! returned unlimited number of times.
//! 4. [`FnConfigurator::returns_with`] - calculates return value on the fly using
//! functions' source arguments. Receives tuple of argument references. Returns unlimited number of
//! times.
//!
//! If multiple return values were specified, then they will be used in the same order:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] fn work(v: i32) -> i32 { v + 1 }
//!
//! # fn main() {
//! // Arrange
//! work::setup(Arg::Any).returns(10)                   // config #1
//!      .setup(Arg::Any).returns_many([20, 30])          // config #2
//!      .setup(Arg::Any).returns_with(|(v,)| *v + 10); // config #3
//!
//! // Act
//! let first  = work(1);   // uses config #1
//! let second = work(2);   // uses config #2
//! let third  = work(3);   // uses config #2
//! let fourth = work(4);   // uses config #3
//! let fifth  = work(5);   // uses config #3
//!
//! // Assert
//! assert_eq!(first,  10);
//! assert_eq!(second, 20);
//! assert_eq!(third,  30);
//! assert_eq!(fourth, 14);
//! assert_eq!(fifth,  15);
//! # }
//! ```
//!
//! ### Callbacks
//! Every mocked function can have a callback that is called when function's configuration is called
//! with matching arguments.
//!
//! If function has return value, then it can be set using [`FnCallbackConfigurator::and_does`]
//! after it's return value was specified. If function does not have return value, then it can be
//! set by calling [`FnConfigurator::does`] straightaway. Static functions receive tuple of argument
//! values in the callback:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] fn get(v: i32) -> i32 { v + 1 }
//! #[mock] fn set(v: i32) {}
//!
//! # fn main() {
//! get::setup(Arg::Any)    // Function with return value
//!     .returns(10)        // must set return value first
//!     .and_does(|(v,)| assert_eq!(*v, 10));
//!
//!                         // Function without return value
//! set::setup(Arg::Any)    // can set callback immediately
//!     .does(|(v,)| assert_eq!(*v, 20));
//! get(10);
//! set(20);
//! # }
//! ```
//! Associated functions receive reference to mock and tuple of argument values in the callback:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock]
//! trait Trait {
//!     fn get(&self) -> i32;
//!     fn work(&self, v: i32);
//! }
//!
//! # fn main() {
//! let mut mock = TraitMock::new();
//! mock.setup()
//!     .get().returns(10)
//!     .work(Arg::Any)
//!     .does(|mock_ref, (v,)| {
//!         assert_eq!(mock_ref.get(), 10);
//!         assert_eq!(*v, 20);
//!     });
//! mock.work(20);
//! # }
//! ```
//!
//! ## Base implementation
//!
//! Mocked functions can use their base implementation in tests. To do so, apply `#[mock]` attribute
//! with `base` argument: `#[mock(base)]`.
//! ```ignore
//! #[mock(base)] trait Trait {}
//! #[mock(base)] impl Struct {}
//! #[mock(base)] fn function() {}
//! ```
//!
//! To tell mock object to use base implementation call `call_base()` on function setup:
//!
//! ```
//! use rsubstitute::*;
//!
//! #[mock] fn dependency() {}
//! #[mock(base)]
//! fn work() {
//!     dependency();   // will be called in test
//! }
//!
//! # fn main() {
//! // Arrange
//! work::setup().call_base();
//!
//! // Act
//! work();
//!
//! // Assert
//! dependency::received(1.time());
//! # }
//! ```
//!
//! Functions that have return values treat `call_base()` as return value configuration, so you can
//! not call any of `returns` functions after enabling base implementation:
//!
//! ```ignore
//! #[mock(base)] fn work() -> i32 { 1 }
//!
//! work::setup().call_base().returns(10);
//!                        // ^^^^^^^ - error, return value is already
//!                        //           provided by base implementation
//! ```
//!
//! Base implementation usage is completely optional, you can mix mocked behavior with base calls.
//! In traits, only functions with default implementation can use `call_base()`:
//! ```
//! use rsubstitute::*;
//!
//! #[mock(base)]
//! trait Trait {
//!     fn dependency(&self);               // no default implementation - can not use `call_base()`
//!     fn work(&self, v: i32) -> i32 {     // has default implementation - can use `call_base()`
//!         self.dependency();
//!         return v + 1;
//!     }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! mock.setup()
//!     .work(10).call_base()
//!     .work(20).returns(30);
//!
//! // Act
//! let first  = mock.work(10);
//! let second = mock.work(20);
//!
//! // Assert
//! assert_eq!(first,  11);
//! assert_eq!(second, 30);
//! mock.received()
//!     .work(10, 1.time())
//!     .work(20, 1.time())
//!     .dependency(1.time());
//! # }
//! ```
//!
//! There is one limitation: all arguments of function must be [`Clone`]able for its implementation
//! to be used in tests. If even single argument does not implement `Clone` you will get compilation
//! error. You'll have to change your code or just use `#[mock]`.
//!
//! ## Verifying calls
//!
//! You can check how exactly mocked function was called. To do it use `received()` followed by
//! descriptions of expected calls:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] fn work() {}
//!
//! # fn main() {
//! work();
//! work::received(1.time());   // checks that `work` was called one time
//! # }
//! ```
//!
//! You can verify exact values of passed arguments using [`Arg`], for more information see
//! [Arguments matching](#arguments-matching).
//!
//! Number of calls is verified using [`Times`] type. It can be defined in several ways:
//! 1. [`Times::Never`] - expects function to never be called with given arguments.
//! 2. [`Times::Once`] and [`Times::Exactly`]`(N)` - expects function to be called exactly once or
//! `N` times respectively.
//! 3. [usize::time] and [usize::times] - syntactic sugar for constructing [`Times::Exactly`]`(N)`
//! from `usize` values like `1.time()` or `2.times()`.
//!
//! You can also check that no calls except the ones you expected were called using
//! `no_other_calls()`:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] fn set(_: i32) {}
//!
//! # fn main() {
//! // Act
//! set(10);
//! set(20);
//!
//! // Assert
//! set::received(10, 1.time())
//!     .received(20, 1.time())
//!     .no_other_calls();  // checks that no other calls were performed
//! # }
//! ```
//!
//! If there were some unvalidated calls when `no_other_calls()` was called, then mock object will
//! panic:
//! ```ignore
//! #[mock]
//! trait Trait { fn set(&self, _: i32); }
//!
//! // Arrange
//! let mut mock = TraitMock::new();
//!
//! // Act
//! mock.set(10);
//! mock.set(20);
//! mock.set(30);
//!
//! // Assert
//! mock.received()
//!     .set(10, 1.time())
//!     .set(20, 1.time())
//!     .no_other_calls();  // will panic because `set(30)` was not validated
//! ```
//!
//! ## Generics
//!
//! `rsubstitute` supports generics in functions, traits and structures. They are transferred to
//! generated mocks as is. Here's a simple generics usage example with trait:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock]
//! trait Trait<T> { fn get(&self) -> T; }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::<i32>::new();
//! mock.setup().get().returns(10);
//!
//! // Act
//! let result = mock.get();
//!
//! // Assert
//! assert_eq!(result, 10);
//! mock.received().get(1.time());
//! # }
//!
//! ```
//!
//! ```rust
//! # use std::marker::PhantomData;
//! # use std::fmt::{Debug, Display};
//! use rsubstitute::*;
//!
//! #[mock]
//! struct Struct<'a, T1: Clone + ToString, T2>
//!     where T2: Debug
//! {
//!     t1: T1,
//!     t2: &'a T2
//! }
//!
//! #[mock(base)]
//! impl<'a, T1: Clone + ToString, T2> Struct<'a, T1, T2>
//!     where T2: Debug
//! {
//!     fn new<'b, T3: Debug + ?Sized>(t1: T1, t2: &'a T2, t3: &'b T3) -> Self
//!         where T3: Display
//!     {
//! Self { t1, t2 }
//!     }
//!
//!     fn get_t1(&self) -> T1 {
//!         self.t1.clone()
//!     }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = Struct::new::<str>(10i32, &[1, 2, 3], "quo vadis");
//! mock.setup().get_t1().returns(20);
//!
//! // Act
//! let result = mock.get_t1();
//!
//! // Assert
//! assert_eq!(result, 20);
//! mock.received().get_t1(1.time());
//! # }
//! ```
//!
//! Static functions use separate configurations for each generics combination:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] fn get<T1, T2: Default>(t1: T1) -> T2 { T2::default() }
//!
//! # fn main() {
//! // Arrange
//! get::setup::<i32, &'static str>(10)
//!     .returns("quo vadis");
//! get::setup::<[i32; 3], &'static str>([1, 2, 3])
//!     .returns("veridis quo");
//!
//! // Act
//! let first:  &str = get(10i32);
//! let second: &str = get([1, 2, 3]);
//!
//! // Assert
//! assert_eq!(first,  "quo vadis");
//! assert_eq!(second, "veridis quo");
//! get::received::<_, &str>(10i32, 1.time());
//! get::received::<_, &str>([1, 2, 3], 1.time());
//! # }
//! ```
//!
//! ## Associated constants and types
//!
//! When mocking trait with assoicated constants and types the mock type exposes them via generics
//! by appending them to the source generics list in the same order in which they are defined in
//! trait:
//! ```rust
//! # use std::fmt::Debug;
//! use rsubstitute::*;
//!
//! #[mock(base)]
//! trait Trait {
//!     type Item: Debug + Default;
//!     const NUMBER: usize;
//!
//!     fn get_item(&self) -> Self::Item;
//!     fn get_number(&self) -> usize {
//!         Self::NUMBER
//!     }
//! }
//!
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::<i32, 3>::new();
//!                          // |    \
//!                          // |     Trait::COUNT
//!                          // Trait::Item
//! mock.setup()
//!     .get_item().returns(10)
//!     .get_number().call_base();
//!
//! // Act
//! let item = mock.get_item();
//! let number = mock.get_number();
//!
//! // Assert
//! assert_eq!(item, 10);
//! assert_eq!(number, 3);
//! mock.received().get_item(1.time());
//! # }
//! ```
//!
//! ## `impl Trait` types
//!
//! Functions that accept or return `impl Trait` can also be mocked. Their mocking is different only
//! in one regard: if mocked function returns `impl Trait`, then it's return type is replaced with
//! `Box<dyn Trait>`. Other than that such functions are mocked as usual:
//! ```rust
//! # use std::fmt::Debug;
//! use rsubstitute::*;
//!
//! #[mock]
//! fn work(v: impl Debug) -> impl ToString { 1 }
//!
//! # fn main() {
//! // Arrange
//! work::setup(Arg::Any).returns(Box::new(20));
//!
//! // Act
//! let result = work("whatever");
//!
//! // Assert
//! assert_eq!(result.to_string(), "20");
//! # }
//! ```
//!
//! There are a couple of limitations:
//! 1. `Trait` in `impl Trait` must be dyn-compatible.
//! 2. Because `Trait` must be dyn-compatible, and [`PartialEq`] is not dyn-compatible, the only way
//! to compare arguments is to use [`Arg::is`]. Alternatively, you can use [`Arg::Any`] if you don't
//! need to check for concerete argument value.
//! 3. Can not use multiple trait bounds like `impl Foo + Bar`, in that case you will need to
//! rewrite your function replacing this argument's type with `T: Foo + Bar`.
//! 4. Can return `impl Trait` from mocked function only if `Trait` is implemented for
//! `Box<dyn Trait>` because under the hood `rsubstitute` replaces return value with
//! `Box<dyn Trait>` in generated implementations.
//! 5. Nested `impl Trait` are not supported. For example, this can't be mocked:
//! `fn f(_: impl IntoIterator<Item = impl Debug>) {}`
//!
//! ## Trait modifiers
//!
//! Only `unsafe` trait modifier is supported. It can be used both in trait definition and in its
//! implementation:
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] unsafe trait Trait {}
//! #[mock] struct Struct;
//! #[mock] unsafe impl Trait for Struct {}
//! # fn main() {}
//! ```
//!
//! ## Function modifiers
//!
//! Following modifiers are supported: `async`, `unsafe` and `extern`, both in standalone and
//! associated functions:
//!
//! ```rust
//! use rsubstitute::*;
//!
//! #[mock] async fn async_dep() {}
//! #[mock] unsafe fn unsafe_dep() {}
//! #[mock] extern "C" fn extern_dep() {}
//! 
//! #[mock(base)]
//! # #[allow(improper_ctypes_definitions)]
//! async unsafe extern "C" fn modifiers() {
//!     async_dep().await;
//!     unsafe_dep();
//!     extern_dep();
//! }
//! 
//! trait Trait {
//! # #[allow(improper_ctypes_definitions)]
//!     async unsafe extern "C" fn modifiers(&self) {
//!         async_dep().await;
//!         unsafe { unsafe_dep(); }
//!         extern_dep();
//!     }
//! }
//! # fn main() {}
//! ```
//! 
//! ## Call order validation
//! 
//! To verify that calls were received in the specific order wrap `received()` assertions inside
//! [`verify_call_order`] callback. This function checks that all assertions inside of it are passed
//! sequentially:
//! ```rust
//! use rsubstitute::*;
//! 
//! #[mock] fn set(_: i32) {}
//! 
//! # fn main() {
//! // Act
//! set(1); set(2); set(3);
//! 
//! // Assert
//! verify_call_order(|| {
//!     set::received(1, 1.time())
//!         .received(2, 1.time())
//!         .received(3, 1.time());
//! });
//! # }
//! ```
//! 
//! If call order is violated, then [`verify_call_order`] will panic after performing all `received`
//! assertions. For example, this panics:
//! ```ignore
//! // Act
//! set(1); set(3); set(2);
//! 
//! // Assert
//! verify_call_order(|| {
//!     set::received(1, 1.time())
//!         .received(2, 1.time())
//!         .received(3, 1.time());
//! });
//! ```
//! 
//! Call order is verified for all mocked functions relative to each, regardless if they're the same
//! function:
//! ```rust
//! use rsubstitute::*;
//! 
//! #[mock] fn first() {}
//! #[mock] fn second() {}
//! #[mock] fn third() {}
//! 
//! # fn main() {
//! // Act
//! first(); second(); third();
//! 
//! // Assert
//! verify_call_order(|| {
//!      first::received(1.time());
//!     second::received(1.time());
//!      third::received(1.time());
//! });
//! # }
//! ```
//! 
//! ## Receiver types
//! 
//! Associated functions can have any kind of receiver type: `&Self`, `Self`, `Rc<Self>` or even
//! nested types like `Box<Rc<Arc<&Self>>>`:
//! ```rust
//! # use std::rc::Rc;
//! # use std::sync::Arc;
//! use rsubstitute::*;
//! 
//! #[mock]
//! trait Trait {
//!     fn fn_self(self);
//!     fn fn_ref_self(&self);
//!     fn fn_rc_self(self: Rc<Self>);
//!     fn fn_nested_self(self: Box<Rc<Arc<&Self>>>);
//! }
//! 
//! # fn main() {}
//! ```
//! 
//! To mock them you don't need to put mock in the same container as source function:
//! ```rust
//! use rsubstitute::*;
//! 
//! #[mock]
//! trait Trait {
//!     fn boxed(self: Box<Self>) -> i32;
//! }
//! 
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! mock.setup().boxed().returns(10);
//! 
//! // Act
//! let result = Box::new(mock.clone()).boxed();    // another feature - mocks cloning, more
//!                                                 // information in "Cloning mocks" section
//! 
//! // Assert
//! assert_eq!(result, 10);
//! mock.received().boxed(1.time());
//! # }
//! ```
//! 
//! ## Cloning mocks
//! 
//! Mock can be cloned either if:
//! 1. it is mock of trait,
//! 2. it is mock struct that has `#[derive(Clone)]` attribute (manually implementing [`Clone`]
//! won't work).
//! 
//! Mock clones share same configuration (it is stored behind reference-counted pointer internally).
//! This let's you share mocks between parts of your code. This maybe useful, for example, if you
//! want to verify that mock received consuming function:
//! ```rust
//! use rsubstitute::*;
//! 
//! #[mock]
//! trait Trait {
//!     fn consume(self);
//! }
//! 
//! # fn main() {
//! // Arrange
//! let mut mock = TraitMock::new();
//! let mut mock_for_verification = mock.clone();
//! 
//! // Act
//! mock.consume();
//! 
//! // Assert
//! mock_for_verification.received().consume(1.time());
//! # }
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
pub use rsubstitute_core::infrastructure::{FnCallbackConfigurator, FnConfigurator};
pub use rsubstitute_core::verify_call_order;
pub use rsubstitute_core::*;

pub use rsubstitute_core::infrastructure::Mockable;

pub mod for_generated {
    pub use rsubstitute_core::args::*;
    pub use rsubstitute_core::fn_parameters::*;
    pub use rsubstitute_core::infrastructure::*;
    pub use rsubstitute_core::*;
}