use crate::arguments_matching::IArgsMatcher;
use crate::{FnConfig, Times};
use std::cell::RefCell;
use std::rc::Rc;

pub struct FnData<TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue> {
    calls: RefCell<Vec<TCall>>,
    configs: RefCell<Vec<Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>>>>,
}

impl<TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue> Default
    for FnData<TCall, TArgsMatcher, TReturnValue>
{
    fn default() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            configs: RefCell::new(Vec::new()),
        }
    }
}

impl<TCall: Clone, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue: Clone>
    FnData<TCall, TArgsMatcher, TReturnValue>
{
    pub fn register_call(&self, call: TCall) -> &Self {
        self.calls.borrow_mut().push(call);
        self
    }

    pub fn add_config(
        &self,
        args_matcher: TArgsMatcher,
    ) -> Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>> {
        let config = FnConfig::new(args_matcher);
        let shared_config = Rc::new(RefCell::new(config));
        self.configs.borrow_mut().push(shared_config.clone());
        return shared_config;
    }

    pub fn handle(&self, call: TCall) {
        let maybe_fn_config = self.try_get_matching_config(call.clone());
        self.register_call(call.clone());
        if let Some(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call);
            if let Some(callback) = fn_config.borrow().get_callback() {
                callback();
            }
        }
    }

    pub fn handle_returning(&self, call: TCall) -> TReturnValue {
        let fn_config = self
            .try_get_matching_config(call.clone())
            .expect("No fn configuration found for this call! TODO: write call description");
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call);
        if let Some(callback) = fn_config.borrow().get_callback() {
            callback();
        }
        let return_value = fn_config
            .borrow_mut()
            .get_return_value()
            .expect("No return value configured for 'another_work'! TODO: write call description?");
        return return_value;
    }

    pub fn verify_received(&self, args_matcher: TArgsMatcher, times: Times) {
        let calls = self.calls.borrow();
        let calls_matching_result: Vec<_> = calls
            .iter()
            .map(|call| args_matcher.matches((*call).clone()))
            .collect();
        let matching_calls_count = calls_matching_result
            .iter()
            .filter(|x| x.iter().all(|y| y.is_none()))
            .count();
        // let matching_calls_count = calls
        //     .iter()
        //     .filter(|call| args_matcher.matches((*call).clone()))
        //     .count();

        let valid = times.matches(matching_calls_count);
        if !valid {
            if matching_calls_count > 0 {
                panic!(
                    "Expected 'work' to be called {times}, but it was called {matching_calls_count} times."
                );
            }
            panic!("Expected 'work' to be called {times}, but it was never called.");
        }
    }

    fn try_get_matching_config(
        &self,
        call: TCall,
    ) -> Option<Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>>> {
        let configs = self.configs.borrow();
        let maybe_fn_config = configs
            .iter()
            .find(|config| config.borrow().matches(call.clone()))
            .cloned();
        return maybe_fn_config;
    }
}
