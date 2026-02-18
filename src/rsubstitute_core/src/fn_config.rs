use crate::IBaseCaller;
use crate::args::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct FnConfig<TMock, TCall, TReturnType, TArgsChecker> {
    _phantom_mock: PhantomData<TMock>,
    args_checker: TArgsChecker,
    current_return_value_index: usize,
    return_values: VecDeque<TReturnType>,
    calls: Vec<Arc<TCall>>,
    callback: Option<Arc<RefCell<dyn FnMut()>>>,
    call_base: bool,
}

impl<TMock, TCall, TReturnType, TArgsChecker> FnConfig<TMock, TCall, TReturnType, TArgsChecker> {
    pub(crate) fn new(args_checker: TArgsChecker) -> Self {
        FnConfig {
            _phantom_mock: PhantomData,
            args_checker,
            current_return_value_index: 0,
            return_values: VecDeque::new(),
            calls: Vec::new(),
            callback: None,
            call_base: false,
        }
    }

    pub(crate) fn add_return_value(&mut self, return_value: TReturnType) {
        self.return_values.push_back(return_value);
    }

    pub(crate) fn add_return_values<const N: usize>(&mut self, return_values: [TReturnType; N]) {
        self.return_values.extend(return_values.into_iter());
    }

    pub(crate) fn set_callback(&mut self, callback: impl FnMut() + 'static) {
        self.callback = Some(Arc::new(RefCell::new(callback)));
    }

    pub(crate) fn register_call(&mut self, call: Arc<TCall>) {
        self.calls.push(call);
    }

    pub(crate) fn check_call(&self, call: &TCall) -> Vec<ArgCheckResult>
    where
        TArgsChecker: IArgsChecker<TCall>,
    {
        self.args_checker.check(&call)
    }

    pub(crate) fn select_next_return_value(&mut self) -> Option<TReturnType>
    where
        TReturnType: Clone,
    {
        let return_value = self
            .return_values
            .get(self.current_return_value_index)
            .cloned();
        if return_value.is_some() {
            let new_current_return_value_index =
                (self.current_return_value_index + 1).min(self.return_values.len() - 1);
            self.current_return_value_index = new_current_return_value_index;
        }
        return return_value;
    }

    pub(crate) fn get_callback(&self) -> Option<Arc<RefCell<dyn FnMut()>>> {
        self.callback.clone()
    }
}

impl<TMock, TCall, TReturnType, TArgsChecker> FnConfig<TMock, TCall, TReturnType, TArgsChecker>
where
    TMock: IBaseCaller<TCall, TReturnType>,
{
    pub(crate) fn set_call_base(&mut self) {
        self.call_base = true;
    }

    pub(crate) fn should_call_base(&self) -> bool {
        self.call_base
    }
}
