use crate::args_matching::IArgsMatcher;
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
        let calls = self.calls.borrow();
        let args_matching_results: Vec<_> = calls
            .iter()
            .map(|call| args_matcher.matches((*call).clone()))
            .collect();
        let matching_calls_count = args_matching_results
            .iter()
            .filter(|args_matching_result| {
                args_matching_result
                    .iter()
                    .all(|arg_matching_result| arg_matching_result.is_ok())
            })
            .count();

        let valid = times.matches(matching_calls_count);
        if !valid {
            let mut not_matching_calls_grouped_by_errors_count: Vec<_> = args_matching_results
                .into_iter()
                .filter_map(|args_matching_result| {
                    let errors_count = args_matching_result
                        .iter()
                        .filter(|arg_matching_result| arg_matching_result.is_err())
                        .count();
                    return if errors_count == 0 {
                        None
                    } else {
                        Some((errors_count, args_matching_result))
                    };
                })
                .collect();
            not_matching_calls_grouped_by_errors_count.sort_by(|a, b| a.0.cmp(&b.0));
            let not_matching_calls_count = not_matching_calls_grouped_by_errors_count.len();
            let not_matching_calls_ordered_by_errors_count: Vec<_> =
                not_matching_calls_grouped_by_errors_count
                    .into_iter()
                    .map(|x| x.1)
                    .collect();
            let not_matching_calls_str = not_matching_calls_ordered_by_errors_count
                .into_iter()
                .take(Self::MAX_INVALID_CALLS_LISTED_COUNT)
                .enumerate()
                .map(|(i, x)| {
                    let call_number = i + 1;
                    let error_msgs = x
                        .into_iter()
                        .map(|y| format!("{y:?}"))
                        .collect::<Vec<_>>()
                        .join("\n");
                    format!("Call #{call_number}\n{error_msgs}")
                })
                .collect::<Vec<_>>()
                .join("\n");
            panic!(
                r#"Expected 'work' to be called {times}, but it was called {matching_calls_count} times.
Expected arguments: {args_matcher:?}
Received {not_matching_calls_count} not matching calls:
{not_matching_calls_str}"#
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
}
