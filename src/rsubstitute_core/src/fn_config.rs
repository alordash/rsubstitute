use crate::args_matching::{ArgCheckResult, IArgsChecker};
use std::collections::VecDeque;

pub struct FnConfig<TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue> {
    args_checker: TArgsChecker,
    return_values: VecDeque<TReturnValue>,
    callback: Option<Box<dyn FnMut()>>,
    calls: Vec<TCall>,
}

impl<TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue: Clone>
    FnConfig<TCall, TArgsChecker, TReturnValue>
{
    pub fn new(args_checker: TArgsChecker) -> Self {
        FnConfig {
            args_checker,
            return_values: VecDeque::new(),
            callback: None,
            calls: Vec::new(),
        }
    }

    pub fn add_return_value(&mut self, return_value: TReturnValue) {
        self.return_values.push_back(return_value);
    }

    pub fn add_return_values(&mut self, return_values: &[TReturnValue]) {
        for return_value in return_values.iter().cloned() {
            self.add_return_value(return_value);
        }
    }

    pub fn set_callback(&mut self, callback: impl FnMut() + 'static) {
        self.callback = Some(Box::new(callback));
    }

    pub fn register_call(&mut self, call: TCall) {
        self.calls.push(call);
    }

    pub fn check(&'_ self, call: TCall) -> Vec<ArgCheckResult<'_>> {
        self.args_checker.check(call)
    }

    pub fn take_return_value(&mut self) -> Option<TReturnValue> {
        self.return_values.pop_front()
    }

    pub fn get_callback(&mut self) -> Option<&mut Box<dyn FnMut()>> {
        self.callback.as_mut()
    }
}
