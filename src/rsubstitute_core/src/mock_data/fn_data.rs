use crate::args::*;
use crate::fn_parameters::*;
use crate::matching_config_search_result::*;
use crate::mock_data::*;
use crate::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

pub struct FnData<'rs, TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool> {
    fn_name: &'static str,
    pub call_infos: RefCell<HashMap<GenericsHashKey, Vec<CallCheck<'rs>>>>,
    pub configs: RefCell<HashMap<GenericsHashKey, Vec<Arc<RefCell<FnConfig<'rs, TMock>>>>>>,
}

impl<'rs, TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool>
    FnData<'rs, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>
{
    pub fn new(fn_name: &'static str) -> Self {
        Self {
            fn_name,
            call_infos: RefCell::new(HashMap::new()),
            configs: RefCell::new(HashMap::new()),
        }
    }

    pub fn reset(&self) {
        self.call_infos.borrow_mut().clear();
        self.configs.borrow_mut().clear();
    }

    pub fn add_config<
        'a,
        TArgsChecker: IArgsChecker + 'a,
        TOwner,
        TArgRefsTuple: Copy,
        TReturnValue,
    >(
        &self,
        args_checker: TArgsChecker,
        fn_tuner_owner: &'a TOwner,
    ) -> FnTuner<
        'a,
        TMock,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        SUPPORTS_BASE_CALLING,
        STORES_MOCK_DATA,
    > {
        let dyn_args_checker: DynArgsChecker<'a> = DynArgsChecker::new(args_checker);
        let generics_hash_key = dyn_args_checker.get_generics_hash_key();
        let config = FnConfig::<'a>::new(dyn_args_checker);
        let arc_config = Arc::new(RefCell::new(config));
        self.configs
            .borrow_mut()
            .entry(generics_hash_key)
            .or_default()
            .push(transmute_lifetime!(arc_config.clone()));
        let fn_tuner = FnTuner::new(arc_config, fn_tuner_owner);
        return fn_tuner;
    }

    pub fn verify_received<'a, TArgsChecker: IArgsChecker + 'a>(
        &self,
        args_checker: TArgsChecker,
        times: Times,
    ) {
        let dyn_args_checker = DynArgsChecker::new(args_checker);
        let (matching_calls_check_result, non_matching_calls_check_result) =
            self.get_matching_and_non_matching_calls(&dyn_args_checker);
        let matching_calls_count = matching_calls_check_result.calls_args_check_results.len();
        let valid = times.matches(matching_calls_count);
        if !valid {
            error_printing::panic_received_verification_error(
                self.fn_name,
                &dyn_args_checker,
                matching_calls_check_result,
                non_matching_calls_check_result,
                times,
            );
        }
    }

    pub fn get_unexpected_calls_error_msgs(&self) -> Vec<String> {
        let all_call_infos = self.call_infos.borrow();
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
                let call = x.get_call();
                error_printing::format_received_unexpected_call_error(
                    self.fn_name,
                    call.get_arg_infos(),
                    call.get_generic_parameter_infos(),
                )
            })
            .collect();
        return unexpected_call_arg_infos;
    }
}

impl<'rs, TMock, const STORES_MOCK_DATA: bool> FnData<'rs, TMock, false, STORES_MOCK_DATA> {
    pub fn handle<'a, TCall: ICall + 'a>(&self, mock: &TMock, the_call: TCall) {
        let call = Arc::new(DynCall::new(the_call));
        let maybe_fn_config = self.try_get_matching_config(&call);
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call.clone());
            if let Some(callback) = fn_config.borrow().get_callback() {
                callback.borrow_mut()(mock as *const TMock as *const (), call.as_ref());
            }
        }
    }

    pub fn handle_returning<'a, 'b, TCall: ICall + 'a, TReturnValue: IReturnValue<'b>>(
        &self,
        mock: &TMock,
        the_call: TCall,
    ) -> TReturnValue {
        let dyn_call = DynCall::new(the_call);
        let call = Arc::new(dyn_call);
        let fn_config = self.get_required_matching_config(&call);
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        let fn_config_ref = fn_config.borrow();
        if let Some(callback) = fn_config_ref.get_callback() {
            callback.borrow_mut()(mock as *const TMock as *const (), call.as_ref());
        }
        drop(fn_config_ref);
        let Some(return_value) = fn_config.borrow_mut().select_next_return_value(&call) else {
            error_printing::panic_no_return_value_was_configured(
                self.fn_name,
                call.get_arg_infos(),
                call.get_generic_parameter_infos(),
            );
        };
        return return_value.downcast_into();
    }
}

impl<'rs, TMock, const STORES_MOCK_DATA: bool> FnData<'rs, TMock, true, STORES_MOCK_DATA> {
    pub fn handle_base<'a, TCall: ICall + Clone + 'a>(
        &self,
        mock: &TMock,
        the_call: TCall,
        mut base_call: impl FnMut(&TMock, TCall),
    ) {
        let call_for_base_call = the_call.clone();
        let dyn_call = DynCall::new(the_call);
        let call = Arc::new(dyn_call);
        let maybe_fn_config = self.try_get_matching_config(&call);
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call.clone());
            let fn_config_ref = fn_config.borrow();
            if fn_config_ref.should_call_base() {
                base_call(mock, call_for_base_call);
            }
            if let Some(callback) = fn_config_ref.get_callback() {
                callback.borrow_mut()(mock as *const TMock as *const (), call.as_ref());
            }
        }
    }

    pub fn handle_base_returning<
        'a,
        TCall: ICall + Clone + 'a,
        TReturnValue: IReturnValue<'a>,
        TBaseCall: FnMut(&'a TMock, TCall) -> TReturnValue + 'a,
    >(
        &self,
        mock: &'a TMock,
        the_call: TCall,
        mut base_call: TBaseCall,
    ) -> TReturnValue {
        let call_for_base_call = the_call.clone();
        let dyn_call = DynCall::new(the_call);
        let call = Arc::new(dyn_call);
        let fn_config = self.get_required_matching_config(&call);
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        let fn_config_ref = fn_config.borrow();
        if fn_config_ref.should_call_base() {
            let base_return_value = base_call(mock, call_for_base_call);
            return base_return_value;
        }
        if let Some(callback) = fn_config_ref.get_callback() {
            callback.borrow_mut()(mock as *const TMock as *const (), call.as_ref());
        }
        drop(fn_config_ref);
        let Some(return_value) = fn_config.borrow_mut().select_next_return_value(&call) else {
            error_printing::panic_no_return_value_was_configured(
                self.fn_name,
                call.get_arg_infos(),
                call.get_generic_parameter_infos(),
            );
        };
        return return_value.downcast_into();
    }
}

mod internal {
    use super::*;

    impl<'rs, TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool>
        FnData<'rs, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>
    {
        pub(crate) fn register_call(&self, call: Arc<DynCall<'rs>>) -> &Self {
            let generics_hash_key = call.get_generics_hash_key();
            self.call_infos
                .borrow_mut()
                .entry(generics_hash_key)
                .or_default()
                .push(CallCheck::new(call));
            self
        }

        pub(crate) fn get_matching_and_non_matching_calls(
            &self,
            dyn_args_checker: &DynArgsChecker,
        ) -> (CallsCheckResult, CallsCheckResult) {
            let mut matching_calls_args_check_results = Vec::new();
            let mut non_matching_calls_args_check_results = Vec::new();
            let generics_hash_key = dyn_args_checker.get_generics_hash_key();
            let mut all_call_infos = self.call_infos.borrow_mut();
            let specific_call_infos = all_call_infos.entry(generics_hash_key).or_default();
            for call_info in specific_call_infos.iter_mut() {
                let call_args_check_results = dyn_args_checker.check(call_info.get_call());
                let is_matching = call_args_check_results.iter().all(ArgCheckResult::is_ok);
                if is_matching {
                    call_info.mark_as_verified();
                    matching_calls_args_check_results.push(call_args_check_results);
                } else {
                    non_matching_calls_args_check_results.push(call_args_check_results);
                }
            }
            let matching_calls_check_result =
                CallsCheckResult::new(matching_calls_args_check_results);
            let non_matching_calls_check_result =
                CallsCheckResult::new(non_matching_calls_args_check_results);
            return (matching_calls_check_result, non_matching_calls_check_result);
        }

        pub(crate) fn try_get_matching_config(
            &self,
            dyn_call: &DynCall<'rs>,
        ) -> MatchingConfigSearchResult<'rs, TMock> {
            let generics_hash_key = dyn_call.get_generics_hash_key();
            let all_configs = self.configs.borrow();
            let Some(matching_configs) = all_configs.get(&generics_hash_key) else {
                return MatchingConfigSearchResult::Err(MatchingConfigSearchErr::empty());
            };
            let mut calls_args_check_results = Vec::with_capacity(matching_configs.len());
            for config in matching_configs.iter() {
                let args_check_result = config.borrow().check_call(dyn_call);
                if args_check_result.iter().all(|x| x.is_ok()) {
                    return MatchingConfigSearchResult::Ok(config.clone());
                }
                calls_args_check_results.push(args_check_result);
            }
            calls_args_check_results.sort_by(|a, b| {
                let a_matched_args_count = a.iter().filter(|x| x.is_ok()).count();
                let b_matched_args_count = b.iter().filter(|x| x.is_ok()).count();
                return b_matched_args_count.cmp(&a_matched_args_count);
            });
            let calls_check_result = CallsCheckResult::new(calls_args_check_results);
            return MatchingConfigSearchResult::Err(MatchingConfigSearchErr {
                args_check_results_sorted_by_number_of_correctly_matched_args_descending:
                    calls_check_result,
            });
        }

        pub(crate) fn get_required_matching_config(
            &self,
            call: &DynCall<'rs>,
        ) -> Arc<RefCell<FnConfig<'rs, TMock>>> {
            let fn_config = match self.try_get_matching_config(&call) {
                MatchingConfigSearchResult::Ok(matching_config) => matching_config,
                MatchingConfigSearchResult::Err(matching_config_search_err) => {
                    error_printing::panic_no_suitable_fn_configuration_found(
                        self.fn_name,
                        call.get_arg_infos(),
                        call.get_generic_parameter_infos(),
                        matching_config_search_err,
                    )
                }
            };
            return fn_config;
        }
    }
}
