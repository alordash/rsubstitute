use crate::args_matching::*;
use crate::call_info::CallInfo;
use crate::di::ServiceCollection;
use crate::error_printer::IErrorPrinter;
use crate::*;
use std::cell::RefCell;
use std::sync::Arc;

pub struct FnData<
    TCall: IArgInfosProvider,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue,
    TBaseCaller,
> {
    fn_name: &'static str,
    call_infos: RefCell<Vec<CallInfo<TCall>>>,
    configs: *mut Vec<Arc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue, TBaseCaller>>>>,
    error_printer: Arc<dyn IErrorPrinter>,
}

impl<TCall: IArgInfosProvider, TArgsChecker: IArgsChecker<TCall>, TReturnValue, TBaseCaller>
    FnData<TCall, TArgsChecker, TReturnValue, TBaseCaller>
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
    TCall: IArgInfosProvider + Clone,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue: Clone,
    TBaseCaller,
> FnData<TCall, TArgsChecker, TReturnValue, TBaseCaller>
{
    pub fn register_call(&self, call: TCall) -> &Self {
        self.call_infos.borrow_mut().push(CallInfo::new(call));
        self
    }

    pub fn add_config(
        &self,
        args_checker: TArgsChecker,
    ) -> Arc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue, TBaseCaller>>> {
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
        if let Some(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call);
            if let Some(callback) = fn_config.borrow_mut().get_callback() {
                callback.borrow_mut()();
            }
        }
    }

    pub fn handle_returning(&self, call: TCall) -> TReturnValue {
        let fn_config = self
            .try_get_matching_config(call.clone())
            .expect("No fn configuration found for this call! TODO: write call description");
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call);
        if let Some(callback) = fn_config.borrow_mut().get_callback() {
            callback.borrow_mut()();
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

    fn try_get_matching_config(
        &self,
        call: TCall,
    ) -> Option<Arc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue, TBaseCaller>>>> {
        let configs = unsafe { &*self.configs };
        let maybe_fn_config = configs
            .iter()
            .rev()
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
    ) -> (Vec<Vec<ArgCheckResult<'a>>>, Vec<Vec<ArgCheckResult<'a>>>) {
        let mut matching_calls = Vec::new();
        let mut non_matching_calls = Vec::new();
        let mut call_infos = self.call_infos.borrow_mut();
        for call_info in call_infos.iter_mut() {
            call_info.verify();
            let call_matching_result = args_checker.check(call_info.get_call().clone());
            let is_matching = call_matching_result.iter().all(ArgCheckResult::is_ok);
            if is_matching {
                matching_calls.push(call_matching_result);
            } else {
                non_matching_calls.push(call_matching_result);
            }
        }
        return (matching_calls, non_matching_calls);
    }
}

impl<
    TCall: IArgInfosProvider + Clone,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue: Clone,
    TBaseCaller: IBaseCaller<TCall, TReturnValue>,
> FnData<TCall, TArgsChecker, TReturnValue, TBaseCaller>
{
    pub fn handle_base(&self, call: TCall) {
        let maybe_fn_config = self.try_get_matching_config(call.clone());
        self.register_call(call.clone());
        if let Some(fn_config) = maybe_fn_config {
            fn_config.borrow_mut().register_call(call.clone());
            if let Some(call_base) = fn_config.borrow().get_base_caller() {
                call_base.borrow_mut().call_base(call);
            }
            if let Some(callback) = fn_config.borrow().get_callback() {
                callback.borrow_mut()();
            }
        }
    }

    pub fn handle_base_returning(&self, call: TCall) -> TReturnValue {
        let fn_config = self
            .try_get_matching_config(call.clone())
            .expect("No fn configuration found for this call! TODO: write call description");
        self.register_call(call.clone());
        fn_config.borrow_mut().register_call(call.clone());
        if let Some(call_base) = fn_config.borrow().get_base_caller() {
            return call_base.borrow_mut().call_base(call);
        }
        if let Some(callback) = fn_config.borrow().get_callback() {
            callback.borrow_mut()();
        }
        let return_value = fn_config
            .borrow_mut()
            .get_return_value()
            .expect("No return value configured for 'another_work'! TODO: write call description?");
        return return_value;
    }
}
