use crate::args_matching::{ArgMatchingError, ArgMatchingResult};
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
            Arg::Any => write!(f, "({arg_type_name}) any"),
            Arg::Eq(expected_value) => write!(f, "({arg_type_name}) equal to {expected_value:?}"),
            Arg::Is(_) => write!(f, "({arg_type_name}) custom predicate"),
        }
    }
}

impl<T: Debug + PartialOrd> Arg<T> {
    pub fn matches(&self, arg_name: &'static str, actual_value: T) -> ArgMatchingResult {
        let arg_type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if actual_value.eq(expected_value) {
                    None
                } else {
                    Some(ArgMatchingError::new(
                        arg_name,
                        arg_type_name,
                        format!("Expected: {expected_value:?}\nActual: {actual_value:?}"),
                    ))
                }
            }
            Arg::Is(predicate) => {
                let actual_value_fmt = format!("{:?}", actual_value);
                if predicate(actual_value) {
                    None
                } else {
                    Some(ArgMatchingError::new(
                        arg_name,
                        arg_type_name,
                        format!(
                            "Custom predicate didn't match passed value. Received value: {actual_value_fmt}",
                        ),
                    ))
                }
            }
        }
    }
}

impl<'a, T: Debug + ?Sized> Arg<&'a T> {
    pub fn matches_ref(
        &self,
        arg_name: &'static str,
        actual_value: &'a T,
    ) -> Option<ArgMatchingError> {
        let arg_type_name = std::any::type_name::<T>();
        let actual_ptr = std::ptr::from_ref(actual_value);
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if std::ptr::eq(*expected_value, actual_value) {
                    None
                } else {
                    let expected_ptr = std::ptr::from_ref(expected_value);
                    Some(ArgMatchingError::new(
                        arg_name,
                        arg_type_name,
                        format!(
                            "Expected reference (ptr: {expected_ptr:?}): {expected_value:?}\nActual reference (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    ))
                }
            }
            Arg::Is(predicate) => {
                if predicate(actual_value) {
                    None
                } else {
                    Some(ArgMatchingError::new(
                        arg_name,
                        arg_type_name,
                        format!(
                            "Custom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    ))
                }
            }
        }
    }
}

impl<T: Debug + ?Sized> Arg<Rc<T>> {
    pub fn matches_rc(
        &self,
        arg_name: &'static str,
        actual_value: Rc<T>,
    ) -> Option<ArgMatchingError> {
        let arg_type_name = std::any::type_name::<T>();
        let actual_ptr = Rc::as_ptr(&actual_value);
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if Rc::ptr_eq(expected_value, &actual_value) {
                    None
                } else {
                    let expected_ptr = Rc::as_ptr(&expected_value);
                    Some(ArgMatchingError::new(
                        arg_name,
                        arg_type_name,
                        format!(
                            "Expected Rc (ptr: {expected_ptr:?}): {expected_value:?}\nActual reference (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    ))
                }
            }
            Arg::Is(predicate) => {
                if predicate(actual_value.clone()) {
                    None
                } else {
                    Some(ArgMatchingError::new(
                        arg_name,
                        arg_type_name,
                        format!(
                            "Custom predicate didn't match passed Rc. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    ))
                }
            }
        }
    }
}

impl<T: Debug + ?Sized> Arg<Arc<T>> {
    pub fn matches_arc(
        &self,
        arg_name: &'static str,
        actual_value: Arc<T>,
    ) -> Option<ArgMatchingError> {
        let arg_type_name = std::any::type_name::<T>();
        let actual_ptr = Arc::as_ptr(&actual_value);
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if Arc::ptr_eq(expected_value, &actual_value) {
                    None
                } else {
                    let expected_ptr = Arc::as_ptr(&expected_value);
                    Some(ArgMatchingError::new(
                        arg_name,
                        arg_type_name,
                        format!(
                            "Expected Arc (ptr: {expected_ptr:?}): {expected_value:?}\nActual reference (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    ))
                }
            }
            Arg::Is(predicate) => {
                if predicate(actual_value.clone()) {
                    None
                } else {
                    Some(ArgMatchingError::new(
                        arg_name,
                        arg_type_name,
                        format!(
                            "Custom predicate didn't match passed Rc. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    ))
                }
            }
        }
    }
}
