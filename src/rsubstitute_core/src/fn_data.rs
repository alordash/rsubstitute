use crate::args_matching::{ArgMatchingResult, IArgsMatcher};
use crate::error_printer::{ErrorPrinter, IErrorPrinter};
use crate::{FnConfig, Times};
use std::cell::RefCell;
use std::rc::Rc;

pub struct FnData<TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue> {
    fn_name: &'static str,
    calls: RefCell<Vec<TCall>>,
    configs: RefCell<Vec<Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>>>>,
}

impl<TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue>
    FnData<TCall, TArgsMatcher, TReturnValue>
{
    pub fn new(fn_name: &'static str) -> Self {
        Self {
            fn_name,
            calls: RefCell::new(Vec::new()),
            configs: RefCell::new(Vec::new()),
        }
    }
}

impl<TCall: Clone, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue: Clone>
    FnData<TCall, TArgsMatcher, TReturnValue>
{
    // TODO - should be configurable
    const MAX_INVALID_CALLS_LISTED_COUNT: usize = 10;

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
        let (matching_calls, non_matching_calls) =
            self.get_matching_and_non_matching_calls(&args_matcher);
        let matching_calls_count = matching_calls.len();
        let valid = times.matches(matching_calls_count);
        if !valid {
            ErrorPrinter.print_received_verification_error(
                self.fn_name,
                &args_matcher,
                matching_calls,
                non_matching_calls,
                times,
            );
        }
    }

    fn try_get_matching_config(
        &self,
        call: TCall,
    ) -> Option<Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>>> {
        let configs = self.configs.borrow();
        let maybe_fn_config = configs
            .iter()
            .find(|config| {
                config
                    .borrow()
                    .matches(call.clone())
                    .into_iter()
                    .all(|x| x.is_ok())
            })
            .cloned();
        return maybe_fn_config;
    }

    fn get_matching_and_non_matching_calls<'a>(
        &'a self,
        args_matcher: &'a TArgsMatcher,
    ) -> (
        Vec<Vec<ArgMatchingResult<'a>>>,
        Vec<Vec<ArgMatchingResult<'a>>>,
    ) {
        let mut matching_calls = Vec::new();
        let mut non_matching_calls = Vec::new();
        let calls = self.calls.borrow();
        for call in calls.iter() {
            let call_matching_result = args_matcher.matches((*call).clone());
            let is_matching = call_matching_result.iter().all(ArgMatchingResult::is_ok);
            if is_matching {
                matching_calls.push(call_matching_result);
            } else {
                non_matching_calls.push(call_matching_result);
            }
        }
        return (matching_calls, non_matching_calls);
    }
}
