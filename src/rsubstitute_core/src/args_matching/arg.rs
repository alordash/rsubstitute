use crate::args_matching::{ArgCheckResult, ArgCheckResultErr, ArgCheckResultOk, ArgInfo};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::Arc;

struct Private;

pub enum Arg<'a, T> {
    Any,
    Eq(T),
    #[allow(private_interfaces)]
    #[doc(hidden)]
    Is(&'a dyn Fn(T) -> bool, Private),
}

impl<'a, T: Debug> Debug for Arg<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - extract to const field when std::any::type_name becomes stabilized as const fn
        // https://github.com/rust-lang/rust/issues/63084
        let arg_type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => write!(f, "({arg_type_name}): any"),
            Arg::Eq(expected_value) => write!(f, "({arg_type_name}): equal to {expected_value:?}"),
            Arg::Is(_, _) => write!(f, "({arg_type_name}): custom predicate"),
        }
    }
}

impl<'a, T> Arg<'a, T> {
    pub fn is(predicate: impl Fn(T) -> bool + 'a) -> Self {
        let reference = Box::leak(Box::new(predicate));
        return Self::Is(reference, Private);
    }
}

impl<'a, T: Debug + PartialOrd + Clone + 'a> Arg<'a, T> {
    pub fn check(&self, arg_name: &'static str, actual_value: T) -> ArgCheckResult<'a> {
        let arg_info = ArgInfo::new(arg_name, actual_value.clone());
        match self {
            Arg::Eq(expected_value) if !actual_value.eq(expected_value) => {
                return ArgCheckResult::Err(ArgCheckResultErr {
                    arg_info,
                    error_msg: format!(
                        "\t\tExpected: {expected_value:?}\n\t\tActual:   {actual_value:?}"
                    ),
                });
            }
            Arg::Is(predicate, _) => {
                let actual_value_str = format!("{:?}", actual_value);
                if !predicate(actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed value. Received value: {actual_value_str}",
                        ),
                    });
                }
            }
            _ => (),
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<'a, T: Debug + ?Sized> Arg<'a, &'a T> {
    pub fn check_ref(&self, arg_name: &'static str, actual_value: &'a T) -> ArgCheckResult<'a> {
        let arg_info = ArgInfo::new(arg_name, actual_value);
        let actual_ptr = std::ptr::from_ref(actual_value);
        match self {
            Arg::Eq(expected_value) => {
                let expected_ptr = std::ptr::from_ref(*expected_value);
                if !std::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected reference (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual reference   (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::Is(predicate, _) if !predicate(actual_value) => {
                return ArgCheckResult::Err(ArgCheckResultErr {
                    arg_info,
                    error_msg: format!(
                        "\t\tCustom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                    ),
                });
            }
            _ => (),
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<'a, T: Debug + ?Sized + 'a> Arg<'a, Rc<T>> {
    pub fn check_rc(&self, arg_name: &'static str, actual_value: Rc<T>) -> ArgCheckResult<'a> {
        let arg_info = ArgInfo::new(arg_name, actual_value.clone());
        let actual_ptr = Rc::as_ptr(&actual_value);
        match self {
            Arg::Eq(expected_value) => {
                let expected_ptr = Rc::as_ptr(expected_value);
                if !std::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected Rc (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual Rc   (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::Is(predicate, _) => {
                let actual_value_str = format!("{:?}", actual_value);
                if !predicate(actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed Rc. Received value (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            _ => (),
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<'a, T: Debug + ?Sized + 'a> Arg<'a, Arc<T>> {
    pub fn check_arc(&self, arg_name: &'static str, actual_value: Arc<T>) -> ArgCheckResult<'a> {
        let arg_info = ArgInfo::new(arg_name, actual_value.clone());
        let actual_ptr = Arc::as_ptr(&actual_value);
        match self {
            Arg::Eq(expected_value) => {
                let expected_ptr = Arc::as_ptr(expected_value);
                if !std::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected Arc (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual Arc   (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::Is(predicate, _) => {
                let actual_value_str = format!("{:?}", actual_value);
                if !predicate(actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed Rc. Received value (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            _ => (),
        }
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}
