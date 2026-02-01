use crate::args_matching::*;
use crate::call_info::CallInfo;
use crate::di::ServiceCollection;
use crate::error_printer::IErrorPrinter;
use crate::matching_config_search_result::*;
use crate::*;
use std::cell::RefCell;
use std::sync::Arc;

pub struct FnData<TMock, TCall: IArgInfosProvider, TArgsChecker: IArgsChecker<TCall>, TReturnValue>
{
    fn_name: &'static str,
    call_infos: RefCell<Vec<CallInfo<TCall>>>,
    // Behind a raw reference to lift 'static requirement from TCall, TArgsChecker, etc.
    configs: *mut Vec<Arc<RefCell<FnConfig<TMock, TCall, TArgsChecker, TReturnValue>>>>,
    error_printer: Arc<dyn IErrorPrinter>,
}

impl<TMock, TCall: IArgInfosProvider, TArgsChecker: IArgsChecker<TCall>, TReturnValue>
    FnData<TMock, TCall, TArgsChecker, TReturnValue>
{
    pub fn new(fn_name: &'static str, services: &ServiceCollection) -> Self {
        Self {
            fn_name,
            call_infos: RefCell::new(Vec::new()),
            configs: Box::leak(Box::new(Vec::new())) as *mut Vec<_>,
            error_printer: services.error_printer.clone(),
        }
    }

    pub fn reset(&self) {
        self.call_infos.borrow_mut().clear();
        unsafe { (*self.configs).clear() };
    }
}

impl<
    TMock,
    TCall: IArgInfosProvider + Clone,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue: Clone,
> FnData<TMock, TCall, TArgsChecker, TReturnValue>
{
    pub fn register_call(&self, call: TCall) -> &Self {
        self.call_infos.borrow_mut().push(CallInfo::new(call));
        self
    }

    pub fn add_config(
        &self,
        args_checker: TArgsChecker,
    ) -> Arc<RefCell<FnConfig<TMock, TCall, TArgsChecker, TReturnValue>>> {
        let config = FnConfig::new(args_checker);
        let shared_config = Arc::new(RefCell::new(config));
        unsafe {
            (*self.configs).push(shared_config.clone());
        }
        return shared_config;
    }

    pub fn handle(&self, call: TCall) {
        let maybe_fn_config = self.try_get_matching_config(call.clone());
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call);
            if let Some(callback) = fn_config.borrow_mut().get_callback() {
                callback.borrow_mut()();
            }
        }
    }

    pub fn handle_returning(&self, call: TCall) -> TReturnValue {
        let fn_config = self.get_required_matching_config(call.clone());
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        if let Some(callback) = fn_config.borrow_mut().get_callback() {
            callback.borrow_mut()();
        }
        let Some(return_value) = fn_config.borrow_mut().get_return_value() else {
            self.error_printer
                .panic_no_return_value_was_configured(self.fn_name, call.get_arg_infos());
        };
        return return_value;
    }

    pub fn verify_received(&self, args_checker: TArgsChecker, times: Times) {
        let (matching_calls, non_matching_calls) =
            self.get_matching_and_non_matching_calls(&args_checker);
        let matching_calls_count = matching_calls.len();
        let valid = times.matches(matching_calls_count);
        if !valid {
            self.error_printer.panic_received_verification_error(
                self.fn_name,
                &args_checker,
                matching_calls,
                non_matching_calls,
                times,
            );
        }
    }

    pub fn get_unexpected_calls_error_msgs(&self) -> Vec<String> {
        let call_infos = self.call_infos.borrow();
        let unexpected_call_infos: Vec<_> =
            call_infos.iter().filter(|x| x.is_not_verified()).collect();
        if unexpected_call_infos.is_empty() {
            return Vec::new();
        }
        let unexpected_call_arg_infos = unexpected_call_infos
            .into_iter()
            .map(|x| {
                self.error_printer.format_received_unexpected_call_error(
                    self.fn_name,
                    x.get_call().get_arg_infos(),
                )
            })
            .collect();
        return unexpected_call_arg_infos;
    }

    fn get_matching_and_non_matching_calls<'a>(
        &'a self,
        args_checker: &'a TArgsChecker,
    ) -> (Vec<Vec<ArgCheckResult>>, Vec<Vec<ArgCheckResult>>) {
        let mut matching_calls = Vec::new();
        let mut non_matching_calls = Vec::new();
        let mut call_infos = self.call_infos.borrow_mut();
        for call_info in call_infos.iter_mut() {
            let call_matching_result = args_checker.check(call_info.get_call().clone());
            let is_matching = call_matching_result.iter().all(ArgCheckResult::is_ok);
            if is_matching {
                call_info.verify();
                matching_calls.push(call_matching_result);
            } else {
                non_matching_calls.push(call_matching_result);
            }
        }
        return (matching_calls, non_matching_calls);
    }

    fn try_get_matching_config(
        &self,
        call: TCall,
    ) -> MatchingConfigSearchResult<TMock, TCall, TArgsChecker, TReturnValue> {
        let configs = unsafe { &*self.configs };
        dbg!(configs.len());
        let mut args_check_results = Vec::with_capacity(configs.len());
        for config in configs.iter().rev() {
            let args_check_result = config.borrow().check(call.clone());
            if args_check_result.iter().all(|x| x.is_ok()) {
                return MatchingConfigSearchResult::Ok(config.clone());
            }
            args_check_results.push(args_check_result);
        }
        args_check_results.sort_by(|a, b| {
            let a_matched_args_count = a.iter().filter(|x| x.is_ok()).count();
            let b_matched_args_count = b.iter().filter(|x| x.is_ok()).count();
            return b_matched_args_count.cmp(&a_matched_args_count);
        });
        return MatchingConfigSearchResult::Err(MatchingConfigSearchErr {
            args_check_results_sorted_by_number_of_correctly_matched_args_descending:
                args_check_results,
        });
    }

    fn get_required_matching_config(
        &self,
        call: TCall,
    ) -> Arc<RefCell<FnConfig<TMock, TCall, TArgsChecker, TReturnValue>>> {
        let fn_config = match self.try_get_matching_config(call.clone()) {
            MatchingConfigSearchResult::Ok(matching_config) => matching_config,
            MatchingConfigSearchResult::Err(matching_config_search_err) => {
                self.error_printer.panic_no_suitable_fn_configuration_found(
                    self.fn_name,
                    call.get_arg_infos(),
                    matching_config_search_err,
                )
            }
        };
        return fn_config;
    }
}

impl<
    TMock: IBaseCaller<TCall, TReturnValue>,
    TCall: IArgInfosProvider + Clone,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue: Clone,
> FnData<TMock, TCall, TArgsChecker, TReturnValue>
{
    pub fn handle_base(&self, mock: &TMock, call: TCall) {
        let maybe_fn_config = self.try_get_matching_config(call.clone());
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call.clone());
            if fn_config.borrow().should_call_base() {
                mock.call_base(call);
            }
            if let Some(callback) = fn_config.borrow().get_callback() {
                callback.borrow_mut()();
            }
        }
    }

    pub fn handle_base_returning(&self, mock: &TMock, call: TCall) -> TReturnValue {
        let fn_config = self.get_required_matching_config(call.clone());
        self.register_call(call.clone());
        // TODO - ain't too many fn_config.borrows? Maybe borrow once and reuse borrow?
        fn_config.borrow_mut().register_call(call.clone());
        if fn_config.borrow().should_call_base() {
            return mock.call_base(call);
        }
        if let Some(callback) = fn_config.borrow().get_callback() {
            callback.borrow_mut()();
        }
        let Some(return_value) = fn_config.borrow_mut().get_return_value() else {
            self.error_printer
                .panic_no_return_value_was_configured(self.fn_name, call.get_arg_infos());
        };
        return return_value;
    }
}
