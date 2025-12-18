use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

pub enum Arg<T> {
    Any,
    Eq(T),
    Is(fn(T) -> bool),
}

impl<T: Debug + PartialOrd> Arg<T> {
    pub fn matches(&self, actual_value: T) -> Option<String> {
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if actual_value.eq(expected_value) {
                    None
                } else {
                    Some(format!(
                        "Expected: {:?}\nActual: {:?}",
                        expected_value, actual_value
                    ))
                }
            }
            Arg::Is(predicate) => {
                let actual_value_fmt = format!("{:?}", actual_value);
                if predicate(actual_value) {
                    None
                } else {
                    Some(format!(
                        "Custom predicate didn't match passed value. Received value: {actual_value_fmt}"
                    ))
                }
            }
        }
    }
}

impl<'a, T: Debug + ?Sized> Arg<&'a T> {
    pub fn matches_ref(&self, actual_value: &'a T) -> Option<String> {
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => std::ptr::eq(*expected_value, actual_value),
            Arg::Is(predicate) => predicate(actual_value),
        }
    }
}

impl<T: Debug + ?Sized> Arg<Rc<T>> {
    pub fn matches_rc(&self, actual_value: Rc<T>) -> Option<String> {
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => Rc::ptr_eq(expected_value, &actual_value),
            Arg::Is(predicate) => predicate(actual_value),
        }
    }
}

impl<T: Debug + ?Sized> Arg<Arc<T>> {
    pub fn matches_arc(&self, actual_value: Arc<T>) -> Option<String> {
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => Arc::ptr_eq(expected_value, &actual_value),
            Arg::Is(predicate) => predicate(actual_value),
        }
    }
}
