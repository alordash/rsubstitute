use crate::args_matching::{ArcCheckResult, IArgsChecker};
use crate::di::ServiceCollection;
use crate::error_printer::{ErrorPrinter, IErrorPrinter};
use crate::{FnConfig, Times};
use std::cell::RefCell;
use std::rc::Rc;

pub struct FnData<TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue> {
    fn_name: &'static str,
    calls: RefCell<Vec<TCall>>,
    configs: RefCell<Vec<Rc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue>>>>>,
    error_printer: Rc<dyn IErrorPrinter>,
}

impl<TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue>
    FnData<TCall, TArgsChecker, TReturnValue>
{
    pub fn new(fn_name: &'static str, services: &ServiceCollection) -> Self {
        Self {
            fn_name,
            calls: RefCell::new(Vec::new()),
            configs: RefCell::new(Vec::new()),
            error_printer: services.error_printer.clone(),
        }
    }
}

impl<TCall: Clone, TArgsChecker: IArgsChecker<TCall>, TReturnValue: Clone>
    FnData<TCall, TArgsChecker, TReturnValue>
{
    pub fn register_call(&self, call: TCall) -> &Self {
        self.calls.borrow_mut().push(call);
        self
    }

    pub fn add_config(
        &self,
        args_checker: TArgsChecker,
    ) -> Rc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue>>> {
        let config = FnConfig::new(args_checker);
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

    pub fn verify_received(&self, args_checker: TArgsChecker, times: Times) {
        let (matching_calls, non_matching_calls) =
            self.get_matching_and_non_matching_calls(&args_checker);
        let matching_calls_count = matching_calls.len();
        let valid = times.matches(matching_calls_count);
        if !valid {
            self.error_printer.print_received_verification_error(
                self.fn_name,
                &args_checker,
                matching_calls,
                non_matching_calls,
                times,
            );
        }
    }

    fn try_get_matching_config(
        &self,
        call: TCall,
    ) -> Option<Rc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue>>>> {
        let configs = self.configs.borrow();
        let maybe_fn_config = configs
            .iter()
            .find(|config| {
                config
                    .borrow()
                    .check(call.clone())
                    .into_iter()
                    .all(|x| x.is_ok())
            })
            .cloned();
        return maybe_fn_config;
    }

    fn get_matching_and_non_matching_calls<'a>(
        &'a self,
        args_checker: &'a TArgsChecker,
    ) -> (Vec<Vec<ArcCheckResult<'a>>>, Vec<Vec<ArcCheckResult<'a>>>) {
        let mut matching_calls = Vec::new();
        let mut non_matching_calls = Vec::new();
        let calls = self.calls.borrow();
        for call in calls.iter() {
            let call_matching_result = args_checker.check((*call).clone());
            let is_matching = call_matching_result.iter().all(ArcCheckResult::is_ok);
            if is_matching {
                matching_calls.push(call_matching_result);
            } else {
                non_matching_calls.push(call_matching_result);
            }
        }
        return (matching_calls, non_matching_calls);
    }
}
