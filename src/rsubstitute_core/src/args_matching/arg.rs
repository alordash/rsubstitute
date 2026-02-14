use crate::args_matching::{ArgCheckResult, ArgCheckResultErr, ArgCheckResultOk, ArgInfo};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::Arc;

struct Private;

pub enum Arg<T> {
    Any,
    Eq(T),
    NotEq(T),
    // Private for cleaner API: just pass closure without having to box or reference it.
    #[allow(private_interfaces)]
    #[doc(hidden)]
    // TODO - add ability to pass closure straight-away like in `mockiato`:
    // |arg| arg.partial_eq("Paul"), |arg| arg.any()
    PrivateIs(Box<dyn Fn(&T) -> bool>, Private),
}

impl<T> From<T> for Arg<T> {
    fn from(value: T) -> Self {
        Self::Eq(value)
    }
}

impl<T: Debug> Debug for Arg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - extract to const field when std::any::type_name becomes stabilized as const fn
        // https://github.com/rust-lang/rust/issues/63084
        let arg_type_name = std::any::type_name::<T>();
        match self {
            Arg::Any => write!(f, "({arg_type_name}): any"),
            Arg::Eq(expected_value) => write!(f, "({arg_type_name}): equal to {expected_value:?}"),
            Arg::NotEq(not_expected_value) => {
                write!(f, "({arg_type_name}): NOT equal to {not_expected_value:?}")
            }
            Arg::PrivateIs(_, _) => write!(f, "({arg_type_name}): custom predicate"),
        }
    }
}

impl<T> Arg<T> {
    #[allow(non_snake_case)] // beautify API ✨
    pub fn Is<'a, TFn: Fn(&T) -> bool + 'a>(predicate: TFn) -> Self {
        let reference = Box::new(predicate) as Box<dyn Fn(&T) -> bool + 'a>;
        let static_reference: Box<dyn Fn(&T) -> bool + 'static> =
            unsafe { std::mem::transmute(reference) };
        return Self::PrivateIs(static_reference, Private);
    }
}

impl<T: Debug + PartialOrd + Clone> Arg<T> {
    pub fn check<'a>(&self, arg_name: &'static str, actual_value: &T) -> ArgCheckResult
    where
        T: 'a,
    {
        let arg_info = ArgInfo::new(arg_name, (*actual_value).clone());
        match self {
            Arg::Eq(expected_value) => {
                if !actual_value.eq(expected_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tExpected: {expected_value:?}\n\t\tActual:   {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::NotEq(not_expected_value) => {
                if actual_value.eq(not_expected_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!("\t\tDid not expect to be {not_expected_value:?}"),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
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
            Arg::Any => (),
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<'a, T: Debug + ?Sized> Arg<&'a T> {
    pub fn check_ref(&self, arg_name: &'static str, actual_value: &&'a T) -> ArgCheckResult {
        let arg_info = ArgInfo::new(arg_name, *actual_value);
        let actual_ptr = std::ptr::from_ref(*actual_value);
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
            Arg::NotEq(not_expected_value) => {
                let not_expected_ptr = std::ptr::from_ref(*not_expected_value);
                if std::ptr::eq(actual_ptr, not_expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tDid not expect reference (ptr: {not_expected_ptr:?}): {not_expected_value:?}"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                if !predicate(actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::Any => {}
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<T: Debug + ?Sized> Arg<*mut T> {
    pub fn check_mut(&self, arg_name: &'static str, actual_value: &*mut T) -> ArgCheckResult {
        self.check(arg_name, actual_value)
    }
}

impl<'a, T: Debug + ?Sized> Arg<&'a mut T> {
    pub fn check_mut(
        &self,
        arg_name: &'static str,
        actual_value_ptr: &*mut T,
    ) -> ArgCheckResult {
        let actual_value = unsafe {
            &(*actual_value_ptr)
                .as_ref()
                .expect("Mutable reference to call argument should not be null.")
        };
        let mut_actual_value = unsafe {
            &(*actual_value_ptr)
                .as_mut()
                .expect("Mutable reference to call argument should not be null.")
        };
        let arg_info = ArgInfo::new(arg_name, *actual_value);
        let actual_ptr = std::ptr::from_ref(*actual_value);
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
            Arg::NotEq(not_expected_value) => {
                let not_expected_ptr = std::ptr::from_ref(*not_expected_value);
                if std::ptr::eq(actual_ptr, not_expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tDid not expect reference (ptr: {not_expected_ptr:?}): {not_expected_value:?}"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                if !predicate(mut_actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                        ),
                    });
                }
            }
            Arg::Any => {}
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<T: Debug + ?Sized> Arg<Rc<T>> {
    pub fn check_rc<'a>(&self, arg_name: &'static str, actual_value: &Rc<T>) -> ArgCheckResult
    where
        T: 'a,
    {
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
            Arg::NotEq(not_expected_value) => {
                let not_expected_ptr = Rc::as_ptr(not_expected_value);
                if std::ptr::eq(actual_ptr, not_expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tDid not expect Rc (ptr: {not_expected_ptr:?}): {not_expected_value:?}"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
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
            Arg::Any => {}
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<T: Debug + ?Sized> Arg<Arc<T>> {
    pub fn check_arc<'a>(&self, arg_name: &'static str, actual_value: &Arc<T>) -> ArgCheckResult
    where
        T: 'a,
    {
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
            Arg::NotEq(not_expected_value) => {
                let not_expected_ptr = Arc::as_ptr(not_expected_value);
                if std::ptr::eq(actual_ptr, not_expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tDid not expect Arc (ptr: {not_expected_ptr:?}): {not_expected_value:?}"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                let actual_value_str = format!("{:?}", actual_value);
                if !predicate(actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            "\t\tCustom predicate didn't match passed Arc. Received value (ptr: {actual_ptr:?}): {actual_value_str}"
                        ),
                    });
                }
            }
            Arg::Any => {}
        }
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}
