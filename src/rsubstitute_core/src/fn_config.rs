use crate::args_matching::{ArgMatchingResult, IArgsChecker};

pub struct FnConfig<TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue> {
    args_checker: TArgsChecker,
    return_value: Option<TReturnValue>,
    callback: Option<fn()>,
    calls: Vec<TCall>,
}

impl<TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue: Clone>
    FnConfig<TCall, TArgsChecker, TReturnValue>
{
    pub fn new(args_checker: TArgsChecker) -> Self {
        FnConfig {
            args_checker,
            return_value: None,
            callback: None,
            calls: Vec::new(),
        }
    }

    pub fn set_return_value(&mut self, return_value: TReturnValue) {
        self.return_value = Some(return_value);
    }

    pub fn set_callback(&mut self, callback: fn()) {
        self.callback = Some(callback);
    }

    pub fn register_call(&mut self, call: TCall) {
        self.calls.push(call);
    }

    pub fn matches(&self, call: TCall) -> Vec<ArgMatchingResult> {
        self.args_checker.matches(call)
    }

    pub fn get_return_value(&mut self) -> Option<TReturnValue> {
        self.return_value.clone()
    }

    pub fn get_callback(&self) -> Option<fn()> {
        self.callback
    }
}
