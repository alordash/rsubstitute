use crate::args::*;
use crate::error_printer::IErrorPrinter;
use crate::fn_parameters::*;
use crate::matching_config_search_result::*;
use crate::mock_data::FnConfig;
use crate::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

pub struct FnData<'rs, TMock> {
    fn_name: &'static str,
    call_infos: RefCell<HashMap<GenericsHashKey, Vec<CallCheck<'rs>>>>,
    configs: RefCell<HashMap<GenericsHashKey, Vec<Arc<RefCell<FnConfig<'rs, TMock>>>>>>,
    error_printer: Arc<dyn IErrorPrinter>,
}

impl<'rs, TMock> FnData<'rs, TMock> {
    pub fn new(fn_name: &'static str) -> Self {
        Self {
            fn_name,
            call_infos: RefCell::new(HashMap::new()),
            configs: RefCell::new(HashMap::new()),
            error_printer: SERVICES.error_printer.clone(),
        }
    }

    pub fn add_config<TArgsChecker: IArgsChecker + 'rs>(
        &self,
        args_checker: TArgsChecker,
    ) -> Arc<RefCell<FnConfig<'rs, TMock>>> {
        let dyn_args_checker = DynArgsChecker::new(args_checker);
        let generics_hash_key = dyn_args_checker.get_generics_hash_key();
        let config = FnConfig::new(dyn_args_checker);
        let shared_config = Arc::new(RefCell::new(config));
        self.configs
            .borrow_mut()
            .entry(generics_hash_key)
            .or_default()
            .push(shared_config.clone());
        return shared_config;
    }

    pub fn verify_received<TArgsChecker: IArgsChecker + 'rs>(
        &self,
        args_checker: TArgsChecker,
        times: Times,
    ) {
        let dyn_args_checker = DynArgsChecker::new(args_checker);
        let (matching_calls, non_matching_calls) =
            self.get_matching_and_non_matching_calls(&dyn_args_checker);
        let matching_calls_count = matching_calls.len();
        let valid = times.matches(matching_calls_count);
        if !valid {
            self.error_printer.panic_received_verification_error(
                self.fn_name,
                &dyn_args_checker,
                matching_calls,
                non_matching_calls,
                times,
            );
        }
    }

    pub fn get_unexpected_calls_error_msgs(&self) -> Vec<String> {
        // TODO - turn next two lines into method?
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
                self.error_printer.format_received_unexpected_call_error(
                    self.fn_name,
                    x.get_call().get_arg_infos(),
                )
            })
            .collect();
        return unexpected_call_arg_infos;
    }

    pub fn handle<TCall: ICall + 'rs>(&self, the_call: TCall) {
        let call = Arc::new(DynCall::new(the_call));
        let maybe_fn_config = self.try_get_matching_config(&call);
        self.register_call(call.clone());
        if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call);
            if let Some(callback) = fn_config.borrow().get_callback() {
                callback.borrow_mut()();
            }
        }
    }

    pub fn handle_returning<TCall: ICall + 'rs, TReturnValue: IReturnValue<'rs> + 'rs>(
        &self,
        the_call: TCall,
    ) -> TReturnValue {
        let dyn_call = DynCall::new(the_call);
        let call = Arc::new(dyn_call);
        let fn_config = self.get_required_matching_config(&call);
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        let fn_config_ref = fn_config.borrow();
        if let Some(callback) = fn_config_ref.get_callback() {
            callback.borrow_mut()();
        }
        drop(fn_config_ref);
        let Some(return_value) = fn_config.borrow_mut().select_next_return_value() else {
            self.error_printer
                .panic_no_return_value_was_configured(self.fn_name, call.get_arg_infos());
        };
        return return_value.downcast_into();
    }
}

// TODO - support
// impl<'rs, TMock> FnData<'rs, TMock>
// where
//     TMock: IBaseCaller<TCall, TReturnType>
// {
//     pub fn handle_base(&self, mock: &TMock, the_call: TCall) {
//         let call_for_base_call = the_call.clone();
//         let call = Arc::new(the_call);
//         let maybe_fn_config = self.try_get_matching_config(&call);
//         self.register_call(call.clone());
//         if let MatchingConfigSearchResult::Ok(fn_config) = maybe_fn_config {
//             fn_config.borrow_mut().register_call(call.clone());
//             let fn_config_ref = fn_config.borrow();
//             if fn_config_ref.should_call_base() {
//                 mock.call_base(call_for_base_call);
//             }
//             if let Some(callback) = fn_config_ref.get_callback() {
//                 callback.borrow_mut()();
//             }
//         }
//     }
//
//     pub fn handle_base_returning(&self, mock: &TMock, the_call: TCall) -> TReturnType {
//         let call_for_base_call = the_call.clone();
//         let call = Arc::new(the_call);
//         let fn_config = self.get_required_matching_config(&call);
//         self.register_call(call.clone());
//         fn_config.borrow_mut().register_call(call.clone());
//         let fn_config_ref = fn_config.borrow();
//         if fn_config_ref.should_call_base() {
//             let base_return_value = mock.call_base(call_for_base_call);
//             return base_return_value;
//         }
//         if let Some(callback) = fn_config_ref.get_callback() {
//             callback.borrow_mut()();
//         }
//         drop(fn_config_ref);
//         let Some(return_value) = fn_config.borrow_mut().select_next_return_value() else {
//             self.error_printer
//                 .panic_no_return_value_was_configured(self.fn_name, call.get_arg_infos());
//         };
//         return return_value;
//     }
// }

mod internal_api {
    use super::*;

    impl<'rs, TMock> FnData<'rs, TMock> {
        pub fn reset(&self) {
            self.call_infos.borrow_mut().clear();
            self.configs.borrow_mut().clear();
        }

        pub fn register_call(&self, call: Arc<DynCall<'rs>>) -> &Self {
            let generics_hash_key = call.get_generics_hash_key();
            self.call_infos
                .borrow_mut()
                .entry(generics_hash_key)
                .or_default()
                .push(CallCheck::new(call));
            self
        }

        pub fn get_matching_and_non_matching_calls(
            &self,
            dyn_args_checker: &DynArgsChecker,
        ) -> (Vec<Vec<ArgCheckResult>>, Vec<Vec<ArgCheckResult>>) {
            let mut matching_calls = Vec::new();
            let mut non_matching_calls = Vec::new();
            let generics_hash_key = dyn_args_checker.get_generics_hash_key();
            let mut all_call_infos = self.call_infos.borrow_mut();
            let specific_call_infos = all_call_infos.entry(generics_hash_key).or_default();
            for call_info in specific_call_infos.iter_mut() {
                let call_matching_result = dyn_args_checker.check(call_info.get_call());
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

        pub fn try_get_matching_config(
            &self,
            dyn_call: &DynCall<'rs>,
        ) -> MatchingConfigSearchResult<'rs, TMock> {
            let generics_hash_key = dyn_call.get_generics_hash_key();
            let all_configs = self.configs.borrow();
            let Some(matching_configs) = all_configs.get(&generics_hash_key) else {
                return MatchingConfigSearchResult::Err(MatchingConfigSearchErr::empty());
            };
            let mut args_check_results = Vec::with_capacity(matching_configs.len());
            for config in matching_configs.iter().rev() {
                let args_check_result = config.borrow().check_call(dyn_call);
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

        pub fn get_required_matching_config(
            &self,
            call: &DynCall<'rs>,
        ) -> Arc<RefCell<FnConfig<'rs, TMock>>> {
            let fn_config = match self.try_get_matching_config(&call) {
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
}
