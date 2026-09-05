use crate::args::*;
use crate::transmute_lifetime;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

struct Private;

/// Argument matcher, checks whether certain argument value matches some expectation.
///
/// `T` - type of argument.
#[allow(private_interfaces)]
#[repr(C)]
pub enum Arg<T: ?Sized> {
    /// Accepts any possible value.
    Any,
    #[doc(hidden)]
    PrivateEq(ArgCmp<T>, Private),
    #[doc(hidden)]
    PrivateNotEq(ArgCmp<T>, Private),
    #[doc(hidden)]
    PrivateIs(Box<dyn Fn(*const ()) -> bool>, Private),
}

impl<T: PartialEq> From<T> for Arg<T> {
    fn from(value: T) -> Self {
        Arg::eq(value)
    }
}

impl<T: Debug> Debug for Arg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - extract to const field when std::any::type_name becomes stabilized as const fn_info
        // https://github.com/rust-lang/rust/issues/63084
        let arg_type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => write!(f, "({arg_type_name}): any"),
            Arg::PrivateEq(ArgCmp { value, .. }, _) => {
                write!(f, "({arg_type_name}): equal to {value:?}")
            }
            Arg::PrivateNotEq(ArgCmp { value, .. }, _) => {
                write!(f, "({arg_type_name}): NOT equal to {value:?}")
            }
            Arg::PrivateIs(_, _) => write!(f, "({arg_type_name}): custom predicate"),
        }
    }
}

impl<T> Arg<T> {
    pub fn set_print_arg(&mut self, print_arg: String) {
        match self {
            Arg::PrivateEq(arg_cmp, _) => arg_cmp.print_arg = print_arg,
            Arg::PrivateNotEq(arg_cmp, _) => arg_cmp.print_arg = print_arg,
            _ => {}
        }
    }

    pub fn try_get_value(&self) -> Option<&T> {
        match self {
            Arg::PrivateEq(arg_cmp, _) => Some(arg_cmp.value.as_ref()),
            Arg::PrivateNotEq(arg_cmp, _) => Some(arg_cmp.value.as_ref()),
            _ => None,
        }
    }

    /// Checks that argument value matches some predicate.
    pub fn is<'a, TFn: Fn(&T) -> bool + 'a>(predicate: TFn) -> Self {
        let anonymous_predicate = move |ptr: *const ()| {
            // SAFETY: anonymous predicate is called only internally and passed pointer is always
            // created by casting &T.
            let t_ref = unsafe {
                let t_ptr = ptr as *const T;
                t_ptr
                    .as_ref()
                    .expect("Pointer to argument in Arg::is must not be null.")
            };
            return predicate(t_ref);
        };
        let boxed_anonymous_predicate =
            Box::new(anonymous_predicate) as Box<dyn Fn(*const ()) -> bool + 'a>;
        return Self::PrivateIs(transmute_lifetime!(boxed_anonymous_predicate), Private);
    }

    /// Checks that argument value is equal to given value.
    pub fn eq(value: T) -> Self
    where
        T: PartialEq,
    {
        let arg_cmp = ArgCmp {
            print_arg: print_arg(&value),
            value: Box::new(value),
            comparator: PartialEq::eq,
            maybe_deref_info: None,
        };
        return Self::PrivateEq(arg_cmp, Private);
    }

    /// Checks that argument value is NOT equal to given value.
    pub fn not_eq(value: T) -> Self
    where
        T: PartialEq,
    {
        let arg_cmp = ArgCmp {
            print_arg: print_arg(&value),
            value: Box::new(value),
            comparator: PartialEq::eq,
            maybe_deref_info: None,
        };
        return Self::PrivateNotEq(arg_cmp, Private);
    }

    /// Checks that reference of argument value is equal to reference of given value.
    ///
    /// Reference is acquired from [`Deref::deref`].
    pub fn ref_eq<U>(value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        let deref_info = DerefInfo::new(&value);
        let arg_cmp = ArgCmp {
            print_arg: print_arg(&value),
            value: Box::new(value),
            comparator: |a, b| core::ptr::eq(a.deref(), b.deref()),
            maybe_deref_info: Some(deref_info),
        };
        return Self::PrivateEq(arg_cmp, Private);
    }

    /// Checks that reference of argument value is NOT equal to reference of given value.
    ///
    /// Reference is acquired from [`Deref::deref`].
    pub fn ref_not_eq<U>(value: T) -> Self
    where
        T: Deref<Target = U>,
    {
        let deref_info = DerefInfo::new(&value);
        let arg_cmp = ArgCmp {
            print_arg: print_arg(&value),
            value: Box::new(value),
            comparator: |a, b| core::ptr::eq(a.deref(), b.deref()),
            maybe_deref_info: Some(deref_info),
        };
        return Self::PrivateNotEq(arg_cmp, Private);
    }
}

impl<T: ?Sized> Arg<T> {
    #[doc(hidden)]
    pub fn check<'a>(
        &self,
        arg_name: &'static str,
        actual_value: &T,
        actual_value_str: String,
    ) -> ArgCheckResult
    where
        T: 'a,
    {
        let arg_info = ArgInfo::new(arg_name, actual_value, actual_value_str.clone());
        match self {
            Arg::PrivateEq(arg_cmp, _) => {
                if !arg_cmp.is_arg_equal_to(actual_value) {
                    // let expected_value_str = print_arg(arg_cmp.value.as_ref());
                    let expected_value_str = &arg_cmp.print_arg;
                    let PtrInfo {
                        expected_ptr_info_suffix,
                        actual_ptr_info_suffix,
                    } = arg_cmp.get_ptrs_info_suffix(actual_value);
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected{expected_ptr_info_suffix}: {expected_value_str}\n\t\tActual{actual_ptr_info_suffix} {actual_value_str}"
                        ),
                    });
                }
            }
            Arg::PrivateNotEq(arg_cmp, _) => {
                if arg_cmp.is_arg_equal_to(actual_value) {
                    // let not_expected_value_str = print_arg(arg_cmp.value.as_ref());
                    let not_expected_value_str = &arg_cmp.print_arg;
                    let PtrInfo {
                        expected_ptr_info_suffix,
                        ..
                    } = arg_cmp.get_ptrs_info_suffix(actual_value);
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tDid not expect to be {expected_ptr_info_suffix}{not_expected_value_str}"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                if !predicate(actual_value as *const _ as *const ()) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed value. Received value: {actual_value_str}",
                        ),
                    });
                }
            }
            Arg::Any => (),
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<'rs, 'a, T: ?Sized> Arg<&'a T> {
    #[doc(hidden)]
    pub fn check_ref(
        &self,
        arg_name: &'static str,
        actual_value: &&'a T,
        actual_value_str: String,
    ) -> ArgCheckResult {
        let arg_info = ArgInfo::new(arg_name, actual_value, actual_value_str.clone());
        let actual_ptr = core::ptr::from_ref(*actual_value);
        match self {
            Arg::PrivateEq(arg_cmp, _) => {
                let expected_ptr = core::ptr::from_ref(*arg_cmp.value);
                if !core::ptr::eq(actual_ptr, expected_ptr) {
                    // let expected_value_str = print_arg(arg_cmp.value.as_ref());
                    let expected_value_str = &arg_cmp.print_arg;
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected reference (ptr: {expected_ptr:?}): {expected_value_str}\n\t\tActual reference   (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            Arg::PrivateNotEq(arg_cmp, _) => {
                let not_expected_ptr = core::ptr::from_ref(*arg_cmp.value);
                if core::ptr::eq(actual_ptr, not_expected_ptr) {
                    // let not_expected_value_str = print_arg(arg_cmp.value.as_ref());
                    let not_expected_value_str = &arg_cmp.print_arg;
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tDid not expect reference (ptr: {not_expected_ptr:?}): {not_expected_value_str}"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                if !predicate(actual_value as *const _ as *const ()) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            Arg::Any => {}
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<'a, T: ?Sized> Arg<&'a mut T> {
    #[doc(hidden)]
    pub fn check_mut_ref(
        &self,
        arg_name: &'static str,
        actual_value_ptr: &*mut T,
        actual_value_str: String,
    ) -> ArgCheckResult {
        // SAFETY: mut reference are replaced with mut pointers to allow having multiple mutable
        // references. This is needed to expose argument in `Arg::is` predicate as is, i.e. as a
        // mutable and not a regular reference.
        // It's ok to have multiple mutable references to mock arguments, their mutability should
        // not matter for testing with mocks.
        let actual_value = unsafe {
            &(*actual_value_ptr)
                .as_ref()
                .expect("Mutable reference to call argument should not be null.")
        };
        let arg_info = ArgInfo::new(arg_name, actual_value, actual_value_str.clone());
        let actual_ptr = core::ptr::from_ref(*actual_value);
        match self {
            Arg::PrivateEq(arg_cmp, _) => {
                let expected_ptr = core::ptr::from_ref(*arg_cmp.value);
                if !core::ptr::eq(actual_ptr, expected_ptr) {
                    // let expected_value_str = print_arg(arg_cmp.value.as_ref());
                    let expected_value_str = &arg_cmp.print_arg;
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected reference (ptr: {expected_ptr:?}): {expected_value_str}\n\t\tActual reference   (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            Arg::PrivateNotEq(arg_cmp, _) => {
                let not_expected_ptr = core::ptr::from_ref(*arg_cmp.value);
                if core::ptr::eq(actual_ptr, not_expected_ptr) {
                    // let not_expected_value_str = print_arg(arg_cmp.value.as_ref());
                    let not_expected_value_str = &arg_cmp.print_arg;
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tDid not expect reference (ptr: {not_expected_ptr:?}): {not_expected_value_str}"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                if !predicate(actual_value as *const _ as *const ()) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            Arg::Any => {}
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}
