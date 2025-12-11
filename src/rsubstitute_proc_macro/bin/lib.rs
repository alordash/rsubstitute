use std::cell::RefCell;

pub struct FnConfig<TArgsMatcher, TReturnValue> {
    args_matcher: TArgsMatcher,
    return_value: Option<TReturnValue>,
}

impl<TArgsMatcher, TReturnValue> FnConfig<TArgsMatcher, TReturnValue> {
    pub fn new(args_matcher: TArgsMatcher) -> Self {
        FnConfig {
            args_matcher,
            return_value: None,
        }
    }

    pub fn returns(&mut self, return_value: TReturnValue) {
        self.return_value = Some(return_value);
    }
}

pub struct FnData<TCall, TArgsMatcher, TReturnValue> {
    calls: RefCell<Vec<TCall>>,
    configs: RefCell<Vec<FnConfig<TArgsMatcher, TReturnValue>>>,
}

impl<TCall, TArgsMatcher, TReturnValue> Default for FnData<TCall, TArgsMatcher, TReturnValue> {
    fn default() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            configs: RefCell::new(Vec::new()),
        }
    }
}

impl<TCall, TArgsMatcher, TReturnValue> FnData<TCall, TArgsMatcher, TReturnValue> {
    pub fn register_call(&self, call: TCall) -> &Self {
        self.calls.borrow_mut().push(call);
        self
    }

    pub fn add_config(
        &self,
        config: FnConfig<TArgsMatcher, TReturnValue>,
    ) -> &mut FnConfig<TArgsMatcher, TReturnValue> {
        let mut mut_configs = self.configs.borrow_mut();
        mut_configs.push(config);
        return mut_configs.last_mut().unwrap();
    }
}

pub mod argument_matching;
