// Fn parameters (arguments and return values) are boxed and treated as trait objects.
// This is needed because mock object needs to store information about all its fn parameters.
//
// Using real fn parameters types would clutter mock object type with all their generic parameters.
// Basically all functions generic parameters will be flattened and stored in mock object struct.
// For example `trait Trait { fn work<T>(&self, v: i32) -> T }` would make `struct TraitMock { work_data: FnData<i32, T> }.
// This also leads to a problem of overlapping type arguments.
//
// To relieve mock structs from knowing their functions parameters dynamic fn parameters are used.
//
// SAFETY (dyn structs):
// `Dyn*` structs contain some boxed data. Their primary purpose is downcasting to specific types.
// Information about these types is erased during compilation but is kept during code generation.
// Types that are stored in and loaded from `Dyn*` structs are both controlled by procedure macros.
// This guarantees that down- and upcasting are performed safely.
mod call_check;
mod dyn_arg_refs_tuple;
mod dyn_args_checker;
mod dyn_call;
mod dyn_return_value;
mod i_arg_refs_tuple;
mod i_call;
mod i_return_value;
mod return_value_source;

pub(crate) use call_check::*;
pub(crate) use dyn_arg_refs_tuple::*;
pub(crate) use dyn_args_checker::*;
pub(crate) use dyn_return_value::*;
pub(crate) use i_arg_refs_tuple::*;
pub(crate) use i_return_value::*;
pub(crate) use return_value_source::*;

pub use dyn_call::*;
pub use i_call::*;
