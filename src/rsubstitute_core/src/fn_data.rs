use crate::args_matching::*;
use crate::di::ServiceCollection;
use crate::error_printer::IErrorPrinter;
use crate::fn_parameters::{Call, CallInfo};
use crate::matching_config_search_result::*;
use crate::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct FnData<'a, TMock> {
    fn_name: &'static str,
    call_infos: RefCell<HashMap<GenericsHashKey, Vec<CallInfo<'a>>>>,
    configs: RefCell<HashMap<GenericsHashKey, Vec<Arc<RefCell<FnConfig<'a, TMock>>>>>>,
    error_printer: Arc<dyn IErrorPrinter>,
}

impl<'a, TMock> FnData<'a, TMock> {
    pub fn new(fn_name: &'static str, services: &ServiceCollection) -> Self {
        Self {
            fn_name,
            call_infos: RefCell::new(HashMap::new()),
            configs: RefCell::new(HashMap::new()),
            error_printer: services.error_printer.clone(),
        }
    }

    pub fn reset(&self) {
        self.call_infos.borrow_mut().clear();
        self.configs.borrow_mut().clear();
    }

    pub fn as_local<'b>(&self) -> &FnData<'b, TMock> {
        // To allow storing lifetimes to local references.
        // TODO - write somewhere in README that it's ok because all local references in test method are 'static
        // because they live only during unit test method
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, TMock> FnData<'a, TMock> {
    pub fn register_call(&self, call: Call<'a>) -> &Self {
        let generics_hash_key = call.get_generics_hash_key();
        self.call_infos
            .borrow_mut()
            .entry(generics_hash_key)
            .or_default()
            .push(CallInfo::new(call));
        self
    }

    pub fn add_config<TRawArgsChecker: IArgsChecker<'a> + 'a>(
        &self,
        raw_args_checker: TRawArgsChecker,
    ) -> Arc<RefCell<FnConfig<'a, TMock>>> {
        let generics_hash_key = raw_args_checker.get_generics_hash_key();
        let args_checker = ArgsChecker::new(raw_args_checker);
        let config = FnConfig::new(args_checker);
        let shared_config = Arc::new(RefCell::new(config));
        self.configs
            .borrow_mut()
            .entry(generics_hash_key)
            .or_default()
            .push(shared_config.clone());
        return shared_config;
    }

    pub fn handle(&self, call: Call<'a>) {
        let call_ref: &'a Call<'a> = unsafe { std::mem::transmute(&call) };
        let maybe_fn_config = self.try_get_matching_config(call_ref);
        self.register_call(call_ref.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call);
            if let Some(callback) = fn_config.borrow().get_callback() {
                callback.borrow_mut()();
            }
        }
    }

    pub fn handle_returning<TReturnValue: 'static>(&self, call: Call<'a>) -> TReturnValue {
        let fn_config = self.get_required_matching_config(call.clone());
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        let fn_config_ref = fn_config.borrow();
        if let Some(callback) = fn_config_ref.get_callback() {
            callback.borrow_mut()();
        }
        let Some(return_value) = fn_config_ref.get_return_value() else {
            self.error_printer
                .panic_no_return_value_was_configured(self.fn_name, call.get_arg_infos());
        };
        return return_value.downcast_into();
    }

    pub fn verify_received<TRawArgsChecker: IArgsChecker<'a> + 'a>(
        &self,
        raw_args_checker: TRawArgsChecker,
        times: Times,
    ) {
        let args_checker = ArgsChecker::new(raw_args_checker);
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
        // TODO - turn next two lines into method?
        let call_infos_borrow = self.call_infos.borrow();
        let all_call_infos: &'a HashMap<GenericsHashKey, Vec<CallInfo<'a>>> =
            unsafe { std::mem::transmute(call_infos_borrow.deref()) };
        let unexpected_call_infos: Vec<_> = all_call_infos
            .values()
            .flatten()
            .filter(|x| x.is_not_verified())
            .collect();
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

    fn get_matching_and_non_matching_calls(
        &self,
        args_checker: &ArgsChecker<'a>,
    ) -> (Vec<Vec<ArgCheckResult>>, Vec<Vec<ArgCheckResult>>) {
        let mut matching_calls = Vec::new();
        let mut non_matching_calls = Vec::new();
        let generics_hash_key = args_checker.get_generics_hash_key();
        let mut call_infos_borrow = self.call_infos.borrow_mut();
        let call_infos: &'a mut Vec<CallInfo<'a>> =
            unsafe { std::mem::transmute(call_infos_borrow.entry(generics_hash_key).or_default()) };
        for call_info in call_infos.iter_mut() {
            let call_matching_result = args_checker.check(call_info.get_call());
            let is_matching = call_matching_result.iter().all(ArgCheckResult::is_ok);
            if is_matching {
                call_info.mark_as_verified();
                matching_calls.push(call_matching_result);
            } else {
                non_matching_calls.push(call_matching_result);
            }
        }
        return (matching_calls, non_matching_calls);
    }

    fn try_get_matching_config(&self, call: &'a Call<'a>) -> MatchingConfigSearchResult<'a, TMock> {
        let generics_hash_key = call.get_generics_hash_key();
        let all_configs = self.configs.borrow();
        let Some(matching_configs) = all_configs.get(&generics_hash_key) else {
            return MatchingConfigSearchResult::Err(MatchingConfigSearchErr::empty());
        };
        let mut args_check_results = Vec::with_capacity(matching_configs.len());
        for config in matching_configs.iter().rev() {
            let args_check_result = config.borrow().check(call);
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

    fn get_required_matching_config(&self, call: Call<'a>) -> Arc<RefCell<FnConfig<'a, TMock>>> {
        let call_ref: &'a Call<'a> = unsafe { std::mem::transmute(&call) };
        let fn_config = match self.try_get_matching_config(call_ref) {
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

impl<'a, TMock: IBaseCaller> FnData<'a, TMock> {
    pub fn handle_base(&self, mock: &TMock, call: Call<'a>) {
        // TODO - turn into method?
        let call_ref: &'a Call<'a> = unsafe { std::mem::transmute(&call) };
        let maybe_fn_config = self.try_get_matching_config(call_ref);
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call.clone());
            let fn_config_ref = fn_config.borrow();
            if fn_config_ref.should_call_base() {
                mock.call_base(call);
            }
            if let Some(callback) = fn_config_ref.get_callback() {
                callback.borrow_mut()();
            }
        }
    }

    pub fn handle_base_returning<TReturnValue: 'static>(
        &self,
        mock: &TMock,
        call: Call<'a>,
    ) -> TReturnValue {
        let fn_config = self.get_required_matching_config(call.clone());
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        let fn_config_ref = fn_config.borrow();
        if fn_config_ref.should_call_base() {
            let base_return_value = mock.call_base(call);
            return base_return_value.downcast_into();
        }
        if let Some(callback) = fn_config_ref.get_callback() {
            callback.borrow_mut()();
        }
        let Some(return_value) = fn_config_ref.get_return_value() else {
            self.error_printer
                .panic_no_return_value_was_configured(self.fn_name, call.get_arg_infos());
        };
        return return_value.downcast_into();
    }
}
