use crate::args_matching::{ArgInfo, ArgMatchingResult, ArgMatchingResultErr, ArgMatchingResultOk};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::Arc;

pub enum Arg<T> {
    Any,
    Eq(T),
    Is(fn(T) -> bool),
}

impl<T: Debug> Debug for Arg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - extract to const field when std::any::type_name becomes stabilized as const fn
        // https://github.com/rust-lang/rust/issues/63084
        let arg_type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => write!(f, "({arg_type_name}): any"),
            Arg::Eq(expected_value) => write!(f, "({arg_type_name}): equal to {expected_value:?}"),
            Arg::Is(_) => write!(f, "({arg_type_name}): custom predicate"),
        }
    }
}

impl<'a, T: Debug + PartialOrd + Clone + 'a> Arg<T> {
    pub fn matches(&self, arg_name: &'static str, actual_value: T) -> ArgMatchingResult<'a> {
        let arg_info = ArgInfo::new(arg_name, actual_value.clone());
        match self {
            Arg::Eq(expected_value) if !actual_value.eq(expected_value) => {
                return ArgMatchingResult::Err(ArgMatchingResultErr {
                    arg_info,
                    error_msg: format!("\t\tExpected: {expected_value:?}\n\t\tActual:   {actual_value:?}"),
                });
            }
            Arg::Is(predicate) => {
                let actual_value_str = format!("{:?}", actual_value);
                if !predicate(actual_value) {
                    return ArgMatchingResult::Err(ArgMatchingResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed value. Received value: {actual_value_str}",
                        ),
                    });
                }
            }
            _ => (),
        };
        return ArgMatchingResult::Ok(ArgMatchingResultOk { arg_info });
    }
}

impl<'a, T: Debug + ?Sized> Arg<&'a T> {
    pub fn matches_ref(
        &self,
        arg_name: &'static str,
        actual_value: &'a T,
    ) -> ArgMatchingResult<'a> {
        let arg_info = ArgInfo::new(arg_name, actual_value);
        let actual_ptr = std::ptr::from_ref(actual_value);
        match self {
            Arg::Eq(expected_value) => {
                let expected_ptr = std::ptr::from_ref(*expected_value);
                if !std::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgMatchingResult::Err(ArgMatchingResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected reference (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual reference   (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::Is(predicate) if !predicate(actual_value) => {
                return ArgMatchingResult::Err(ArgMatchingResultErr {
                    arg_info,
                    error_msg: format!(
                        "\t\tCustom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                    ),
                });
            }
            _ => (),
        };
        return ArgMatchingResult::Ok(ArgMatchingResultOk { arg_info });
    }
}

impl<'a, T: Debug + ?Sized + 'a> Arg<Rc<T>> {
    pub fn matches_rc(&self, arg_name: &'static str, actual_value: Rc<T>) -> ArgMatchingResult<'a> {
        let arg_info = ArgInfo::new(arg_name, actual_value.clone());
        let actual_ptr = Rc::as_ptr(&actual_value);
        match self {
            Arg::Eq(expected_value) => {
                let expected_ptr = Rc::as_ptr(expected_value);
                if !std::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgMatchingResult::Err(ArgMatchingResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected Rc (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual Rc   (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::Is(predicate) => {
                let actual_value_str = format!("{:?}", actual_value);
                if !predicate(actual_value) {
                    return ArgMatchingResult::Err(ArgMatchingResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed Rc. Received value (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            _ => (),
        };
        return ArgMatchingResult::Ok(ArgMatchingResultOk { arg_info });
    }
}

impl<'a, T: Debug + ?Sized + 'a> Arg<Arc<T>> {
    pub fn matches_arc(
        &self,
        arg_name: &'static str,
        actual_value: Arc<T>,
    ) -> ArgMatchingResult<'a> {
        let arg_info = ArgInfo::new(arg_name, actual_value.clone());
        let actual_ptr = Arc::as_ptr(&actual_value);
        match self {
            Arg::Eq(expected_value) => {
                let expected_ptr = Arc::as_ptr(expected_value);
                if !std::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgMatchingResult::Err(ArgMatchingResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected Arc (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual Arc   (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::Is(predicate) => {
                let actual_value_str = format!("{:?}", actual_value);
                if !predicate(actual_value) {
                    return ArgMatchingResult::Err(ArgMatchingResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed Rc. Received value (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            _ => (),
        }
        return ArgMatchingResult::Ok(ArgMatchingResultOk { arg_info });
    }
}
