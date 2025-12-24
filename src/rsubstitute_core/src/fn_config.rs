use crate::args_matching::{ArgCheckResult, IArgsChecker};

pub struct FnConfig<TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue> {
    args_checker: TArgsChecker,
    return_value: Option<TReturnValue>,
    callback: Option<Box<dyn FnMut()>>,
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

    pub fn set_callback(&mut self, callback: impl FnMut() + 'static) {
        self.callback = Some(Box::new(callback));
    }

    pub fn register_call(&mut self, call: TCall) {
        self.calls.push(call);
    }

    pub fn check(&self, call: TCall) -> Vec<ArgCheckResult> {
        self.args_checker.check(call)
    }

    pub fn get_return_value(&mut self) -> Option<TReturnValue> {
        self.return_value.clone()
    }

    pub fn get_callback(&mut self) -> Option<&mut Box<dyn FnMut()>> {
        self.callback.as_mut()
    }
}
