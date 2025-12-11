use std::cell::RefCell;

pub struct FnConfig<TCall, TReturnValue> {
    predicate: fn(TCall) -> bool,
    return_value: TReturnValue,
}

pub struct FnData<TCall, TReturnValue> {
    calls: RefCell<Vec<TCall>>,
    configs: RefCell<Vec<FnConfig<TCall, TReturnValue>>>,
}

impl<TCall, TReturnValue> Default for FnData<TCall, TReturnValue> {
    fn default() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            configs: RefCell::new(Vec::new()),
        }
    }
}

impl<TCall, TReturnValue> FnData<TCall, TReturnValue> {
    pub fn add_call(&self, call: TCall) -> &Self {
        self.calls.borrow_mut().push(call);
        self
    }

    pub fn add_return_value(&self, return_value: TReturnValue) -> &Self {
        self.return_values.borrow_mut().push(return_value);
        self
    }

    pub fn get_return_value(&self) -> TReturnValue {
        let Some(return_value) = self.return_values.borrow_mut().pop() else {
            panic!("Return value must've been set!");
        };
        return return_value;
    }
}

pub use value_predicate::*;

mod value_predicate {
    pub trait IValuePredicate<T> {
        fn matches(&self, other: T) -> bool;
    }

    // impl<T> IValuePredicate<T> for fn(T) -> bool {
    //     fn matches(&self, other: T) -> bool {
    //         self(other)
    //     }
    // }

    impl<T: PartialEq> IValuePredicate<T> for T {
        fn matches(&self, other: T) -> bool {
            self.eq(&other)
        }
    }

    impl<T, F: Fn(T) -> bool> IValuePredicate<T> for F {
        fn matches(&self, other: T) -> bool {
            self(other)
        }
    }
}
