use std::ops::Deref;
use std::sync::Arc;

pub enum Arg<T> {
    Any,
    Is(fn(T) -> bool),
    Eq(T),
}

trait Quo {}

impl<T: PartialOrd> Arg<T> {
    pub fn matches(&self, value: T) -> bool {
        match self {
            Arg::Any => true,
            Arg::Is(predicate) => predicate(value),
            Arg::Eq(expected_value) => value.eq(expected_value),
        }
    }
}

impl<'a, T: ?Sized> Arg<&'a T> {
    pub fn matches_ref(&self, value: &'a T) -> bool {
        match self {
            Arg::Any => true,
            Arg::Is(predicate) => predicate(value),
            Arg::Eq(expected_value) => std::ptr::eq(*expected_value, value),
        }
    }
}

impl<T: ?Sized> Arg<Arc<T>> {
    pub fn matches_arc(&self, value: Arc<T>) -> bool {
        match self {
            Arg::Any => true,
            Arg::Is(predicate) => predicate(value),
            Arg::Eq(expected_value) => Arc::ptr_eq(expected_value, &value),
        }
    }
}
