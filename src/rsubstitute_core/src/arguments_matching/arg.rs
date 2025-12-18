use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

pub enum Arg<T> {
    Any,
    Eq(T),
    Is(fn(T) -> bool),
}

impl<T: Debug + PartialOrd> Arg<T> {
    pub fn matches(&self, argument_name: &'static str, actual_value: T) -> Option<String> {
        let type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if actual_value.eq(expected_value) {
                    None
                } else {
                    Some(format!(
                        "{argument_name}: {type_name}\nExpected: {expected_value:?}\nActual: {actual_value:?}"
                    ))
                }
            }
            Arg::Is(predicate) => {
                let actual_value_fmt = format!("{:?}", actual_value);
                if predicate(actual_value) {
                    None
                } else {
                    Some(format!(
                        "{argument_name}: {type_name} | Custom predicate didn't match passed value. Received value: {actual_value_fmt}",
                    ))
                }
            }
        }
    }
}

impl<'a, T: Debug + ?Sized> Arg<&'a T> {
    pub fn matches_ref(&self, argument_name: &'static str, actual_value: &'a T) -> Option<String> {
        let type_name = std::any::type_name::<T>();
        let actual_ptr = std::ptr::from_ref(actual_value);
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if std::ptr::eq(*expected_value, actual_value) {
                    None
                } else {
                    let expected_ptr = std::ptr::from_ref(expected_value);
                    Some(format!(
                        "{argument_name}: {type_name}\nExpected reference (ptr: {expected_ptr:?}): {expected_value:?}\nActual reference (ptr: {actual_ptr:?}): {actual_value:?}"
                    ))
                }
            }
            Arg::Is(predicate) => {
                if predicate(actual_value) {
                    None
                } else {
                    Some(format!(
                        "{argument_name}: {type_name} | Custom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                    ))
                }
            }
        }
    }
}

impl<T: Debug + ?Sized> Arg<Rc<T>> {
    pub fn matches_rc(&self, argument_name: &'static str, actual_value: Rc<T>) -> Option<String> {
        let type_name = std::any::type_name::<T>();
        let actual_ptr = Rc::as_ptr(&actual_value);
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if Rc::ptr_eq(expected_value, &actual_value) {
                    None
                } else {
                    let expected_ptr = Rc::as_ptr(&expected_value);
                    Some(format!(
                        "{argument_name}: {type_name}\nExpected Rc (ptr: {expected_ptr:?}): {expected_value:?}\nActual reference (ptr: {actual_ptr:?}): {actual_value:?}"
                    ))
                }
            }
            Arg::Is(predicate) => {
                if predicate(actual_value.clone()) {
                    None
                } else {
                    Some(format!(
                        "{argument_name}: {type_name} | Custom predicate didn't match passed Rc. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                    ))
                }
            }
        }
    }
}

impl<T: Debug + ?Sized> Arg<Arc<T>> {
    pub fn matches_arc(&self, argument_name: &'static str, actual_value: Arc<T>) -> Option<String> {
        let type_name = std::any::type_name::<T>();
        let actual_ptr = Arc::as_ptr(&actual_value);
        match self {
            Arg::Any => None,
            Arg::Eq(expected_value) => {
                if Arc::ptr_eq(expected_value, &actual_value) {
                    None
                } else {
                    let expected_ptr = Arc::as_ptr(&expected_value);
                    Some(format!(
                        "{argument_name}: {type_name}\nExpected Arc (ptr: {expected_ptr:?}): {expected_value:?}\nActual reference (ptr: {actual_ptr:?}): {actual_value:?}"
                    ))
                }
            }
            Arg::Is(predicate) => {
                if predicate(actual_value.clone()) {
                    None
                } else {
                    Some(format!(
                        "{argument_name}: {type_name} | Custom predicate didn't match passed Rc. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                    ))
                }
            }
        }
    }
}
