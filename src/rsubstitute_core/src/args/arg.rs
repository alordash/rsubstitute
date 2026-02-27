use crate::args::*;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::Arc;

struct Private;

#[allow(private_interfaces)]
pub enum Arg<T> {
    Any,
    #[doc(hidden)]
    PrivateEq(ArgCmp<T>, Private),
    #[doc(hidden)]
    PrivateNotEq(ArgCmp<T>, Private),
    #[doc(hidden)]
    // TODO - add ability to pass closure straight-away like in `mockiato`:
    // |arg| arg.partial_eq("Paul"), |arg| arg.any()
    PrivateIs(Box<dyn Fn(&T) -> bool>, Private),
}

struct ArgCmp<T> {
    value: T,
    comparator: fn(&T, &T) -> bool,
}

impl<T> ArgCmp<T> {
    pub fn is_arg_equal_to(&self, other: &T) -> bool {
        (self.comparator)(&self.value, other)
    }
}

impl<T: PartialEq> From<T> for Arg<T> {
    fn from(value: T) -> Self {
        let arg_cmp = ArgCmp {
            value,
            comparator: PartialEq::eq,
        };
        return Self::PrivateEq(arg_cmp, Private);
    }
}

impl<T: Debug> Debug for Arg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO - extract to const field when std::any::type_name becomes stabilized as const fn
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

// Beautify API ✨
impl<T> Arg<T> {
    #[allow(non_snake_case)]
    pub fn Is<'a, TFn: Fn(&T) -> bool + 'a>(predicate: TFn) -> Self {
        let reference = Box::new(predicate) as Box<dyn Fn(&T) -> bool + 'a>;
        let static_reference: Box<dyn Fn(&T) -> bool + 'static> =
            unsafe { core::mem::transmute(reference) };
        return Self::PrivateIs(static_reference, Private);
    }

    #[allow(non_snake_case)]
    pub fn Eq(value: T) -> Self
    where
        T: PartialEq,
    {
        value.into()
    }

    #[allow(non_snake_case)]
    pub fn NotEq(value: T) -> Self
    where
        T: PartialEq,
    {
        value.into()
    }
}

impl<T> Arg<T> {
    pub fn check<'a>(&self, arg_name: &'static str, actual_value: &T) -> ArgCheckResult
    where
        T: 'a,
    {
        let arg_info = ArgInfo::new(
            arg_name,
            actual_value,
            (&ArgPrinter(actual_value)).debug_string(),
        );
        match self {
            Arg::PrivateEq(arg_cmp, _) => {
                if !arg_cmp.is_arg_equal_to(actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        // error_msg: format!(
                        //     "\t\tExpected: {expected_value:?}\n\t\tActual:   {actual_value:?}"
                        // ),
                        error_msg: format!("TODO - debug string"),
                    });
                }
            }
            Arg::PrivateNotEq(arg_cmp, _) => {
                if arg_cmp.is_arg_equal_to(actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        // error_msg: format!("\t\tDid not expect to be {not_expected_value:?}"),
                        error_msg: format!("TODO - debug string"),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                let actual_value_str = (&ArgPrinter(&actual_value)).debug_string();
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

impl<'a, T: ?Sized> Arg<&'a T> {
    pub fn check_ref(&self, arg_name: &'static str, actual_value: &&'a T) -> ArgCheckResult {
        let arg_info = ArgInfo::new(
            arg_name,
            actual_value,
            (&ArgPrinter(*actual_value)).debug_string(),
        );
        let actual_ptr = core::ptr::from_ref(*actual_value);
        match self {
            Arg::PrivateEq(arg_cmp, _) => {
                let expected_ptr = core::ptr::from_ref(arg_cmp.value);
                if !core::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tExpected reference (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual reference   (ptr: {actual_ptr:?}): {actual_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::PrivateNotEq(arg_cmp, _) => {
                let not_expected_ptr = core::ptr::from_ref(arg_cmp.value);
                if core::ptr::eq(actual_ptr, not_expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tDid not expect reference (ptr: {not_expected_ptr:?}): {not_expected_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                if !predicate(actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tCustom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::Any => {}
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<T: ?Sized> Arg<*mut T> {
    pub fn check_mut(&self, arg_name: &'static str, actual_value: &*mut T) -> ArgCheckResult {
        self.check(arg_name, actual_value)
    }
}

impl<'a, T: ?Sized> Arg<&'a mut T> {
    pub fn check_mut(&self, arg_name: &'static str, actual_value_ptr: &*mut T) -> ArgCheckResult {
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
        let arg_info = ArgInfo::new(
            arg_name,
            actual_value,
            (&ArgPrinter(*actual_value)).debug_string(),
        );
        let actual_ptr = core::ptr::from_ref(*actual_value);
        match self {
            Arg::PrivateEq(arg_cmp, _) => {
                let expected_ptr = core::ptr::from_ref(arg_cmp.value);
                if !core::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tExpected reference (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual reference   (ptr: {actual_ptr:?}): {actual_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::PrivateNotEq(arg_cmp, _) => {
                let not_expected_ptr = core::ptr::from_ref(arg_cmp.value);
                if core::ptr::eq(actual_ptr, not_expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tDid not expect reference (ptr: {not_expected_ptr:?}): {not_expected_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                if !predicate(mut_actual_value) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tCustom predicate didn't match passed reference value. Received value (ptr: {actual_ptr:?}): {actual_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::Any => {}
        };
        return ArgCheckResult::Ok(ArgCheckResultOk { arg_info });
    }
}

impl<T: ?Sized> Arg<Rc<T>> {
    pub fn check_rc<'a>(&self, arg_name: &'static str, actual_value: &Rc<T>) -> ArgCheckResult
    where
        T: 'a,
    {
        let arg_info = ArgInfo::new(
            arg_name,
            actual_value,
            (&ArgPrinter(actual_value)).debug_string(),
        );
        let actual_ptr = Rc::as_ptr(&actual_value);
        match self {
            Arg::PrivateEq(arg_cmp, _) => {
                let expected_ptr = Rc::as_ptr(&arg_cmp.value);
                if !core::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tExpected Rc (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual Rc   (ptr: {actual_ptr:?}): {actual_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::PrivateNotEq(arg_cmp, _) => {
                let not_expected_ptr = Rc::as_ptr(&arg_cmp.value);
                if core::ptr::eq(actual_ptr, not_expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tDid not expect Rc (ptr: {not_expected_ptr:?}): {not_expected_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                // let actual_value_str = format!("{:?}", actual_value);
                let actual_value_str = format!("TODO - debug string");
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

impl<T: ?Sized> Arg<Arc<T>> {
    pub fn check_arc<'a>(&self, arg_name: &'static str, actual_value: &Arc<T>) -> ArgCheckResult
    where
        T: 'a,
    {
        let arg_info = ArgInfo::new(
            arg_name,
            actual_value,
            (&ArgPrinter(actual_value)).debug_string(),
        );
        let actual_ptr = Arc::as_ptr(&actual_value);
        match self {
            Arg::PrivateEq(arg_cmp, _) => {
                let expected_ptr = Arc::as_ptr(&arg_cmp.value);
                if !core::ptr::eq(actual_ptr, expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tExpected Arc (ptr: {expected_ptr:?}): {expected_value:?}\n\t\tActual Arc   (ptr: {actual_ptr:?}): {actual_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::PrivateNotEq(arg_cmp, _) => {
                let not_expected_ptr = Arc::as_ptr(&arg_cmp.value);
                if core::ptr::eq(actual_ptr, not_expected_ptr) {
                    return ArgCheckResult::Err(ArgCheckResultErr {
                        arg_info,
                        error_msg: format!(
                            // "\t\tDid not expect Arc (ptr: {not_expected_ptr:?}): {not_expected_value:?}"
                            "TODO - debug string"
                        ),
                    });
                }
            }
            Arg::PrivateIs(predicate, _) => {
                // let actual_value_str = format!("{:?}", actual_value);
                let actual_value_str = format!("TODO - debug string");
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
