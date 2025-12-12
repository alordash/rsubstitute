use std::sync::Arc;

pub enum Arg<T> {
    Any,
    Eq(T),
    Is(fn(T) -> bool),
}

trait Quo {}

impl<T: PartialOrd> Arg<T> {
    pub fn matches(&self, value: T) -> bool {
        match self {
            Arg::Any => true,
            Arg::Eq(expected_value) => value.eq(expected_value),
            Arg::Is(predicate) => predicate(value),
        }
    }
}

impl<'a, T: ?Sized> Arg<&'a T> {
    pub fn matches_ref(&self, value: &'a T) -> bool {
        match self {
            Arg::Any => true,
            Arg::Eq(expected_value) => std::ptr::eq(*expected_value, value),
            Arg::Is(predicate) => predicate(value),
        }
    }
}

impl<T: ?Sized> Arg<Arc<T>> {
    pub fn matches_arc(&self, value: Arc<T>) -> bool {
        match self {
            Arg::Any => true,
            Arg::Eq(expected_value) => Arc::ptr_eq(expected_value, &value),
            Arg::Is(predicate) => predicate(value),
        }
    }
}
