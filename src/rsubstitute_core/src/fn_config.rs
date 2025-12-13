use crate::arguments_matching::IArgsMatcher;
use std::marker::PhantomData;

pub struct FnConfig<TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue> {
    args_matcher: TArgsMatcher,
    return_value: Option<TReturnValue>,
    callback: Option<fn()>,
    calls: Vec<TCall>,
}

impl<TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue: Clone>
    FnConfig<TCall, TArgsMatcher, TReturnValue>
{
    pub fn new(args_matcher: TArgsMatcher) -> Self {
        FnConfig {
            args_matcher,
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

    pub fn matches(&self, call: TCall) -> bool {
        self.args_matcher.matches(call)
    }

    pub fn get_return_value(&mut self) -> Option<TReturnValue> {
        self.return_value.clone()
    }

    pub fn get_callback(&self) -> Option<fn()> {
        self.callback
    }
}
