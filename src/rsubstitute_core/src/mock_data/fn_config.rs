use crate::args::*;
use crate::fn_parameters::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;

// TODO - maybe remove `TMock`? What is it's purpose now that we have dyn fn parameters?
// All information needed to handle certain function is stored in it's `FnConfig`.
// Previously `TMock` was needed only to handle IBaseCaller.
pub struct FnConfig<'rs, TMock> {
    _phantom_mock: PhantomData<TMock>,
    args_checker: DynArgsChecker<'rs>,
    return_value_sources: VecDeque<ReturnValueSource<'rs>>,
    calls: Vec<Arc<DynCall<'rs>>>,
    callback: Option<Arc<RefCell<dyn FnMut()>>>,
    call_base: bool,
}

impl<'rs, TMock> FnConfig<'rs, TMock> {
    pub(crate) fn new(args_checker: DynArgsChecker<'rs>) -> Self {
        FnConfig {
            _phantom_mock: PhantomData,
            args_checker,
            return_value_sources: VecDeque::new(),
            calls: Vec::new(),
            callback: None,
            call_base: false,
        }
    }

    pub(crate) fn add_return_value_source(&mut self, return_value: ReturnValueSource<'rs>) {
        self.return_value_sources.push_back(return_value);
    }

    pub(crate) fn add_return_value_sources(
        &mut self,
        return_values: impl IntoIterator<Item = ReturnValueSource<'rs>>,
    ) {
        self.return_value_sources.extend(return_values.into_iter());
    }

    pub(crate) fn set_callback(&mut self, callback: impl FnMut() + 'static) {
        self.callback = Some(Arc::new(RefCell::new(callback)));
    }

    pub(crate) fn register_call(&mut self, call: Arc<DynCall<'rs>>) {
        self.calls.push(call);
    }

    pub(crate) fn check_call(&self, call: &DynCall<'rs>) -> Vec<ArgCheckResult> {
        self.args_checker.check(&call)
    }

    pub(crate) fn select_next_return_value(&mut self) -> Option<DynReturnValue<'rs>> {
        let Some(return_value_source) = self.return_value_sources.front() else {return None;};
        return match return_value_source {
            ReturnValueSource::SingleTime(_) => {
                let Some(ReturnValueSource::SingleTime(return_value)) =
                    self.return_value_sources.pop_front()
                else {
                    panic!(
                        "Front return value source must be single time because it was just checked."
                    )
                };
                Some(return_value)
            },
            ReturnValueSource::Perpetual(return_value_factory) => {
                let return_value = return_value_factory();
                Some(return_value)
            }
        };
    }

    pub(crate) fn get_callback(&self) -> Option<Arc<RefCell<dyn FnMut()>>> {
        self.callback.clone()
    }

    pub(crate) fn set_call_base(&mut self) {
        self.call_base = true;
    }

    pub(crate) fn should_call_base(&self) -> bool {
        self.call_base
    }
}

// TODO - support
// impl<'rs, TMock, TCall, TReturnType> FnConfig<'rs, TMock>
// where
//     TMock: IBaseCaller<TCall, TReturnType>,
// {
//     pub(crate) fn set_call_base(&mut self) {
//         self.call_base = true;
//     }
//
//     pub(crate) fn should_call_base(&self) -> bool {
//         self.call_base
//     }
// }
