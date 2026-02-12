use crate::IBaseCaller;
use crate::args_matching::{ArgCheckResult, IDynArgsChecker};
use crate::fn_parameters::Call;
use crate::i_mut_ref_clone::IMutRefClone;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

pub struct FnConfig<TMock, TArgsChecker, TReturnValue> {
    _phantom_mock: PhantomData<TMock>,
    args_checker: TArgsChecker,
    current_return_value_index: Cell<usize>,
    return_values: VecDeque<TReturnValue>,
    calls: Vec<Call>,
    callback: Option<Arc<RefCell<dyn FnMut()>>>,
    call_base: bool,
}

impl<TMock, TArgsChecker, TReturnValue> FnConfig<TMock, TArgsChecker, TReturnValue> {
    pub fn new(args_checker: TArgsChecker) -> Self {
        FnConfig {
            _phantom_mock: PhantomData,
            args_checker,
            current_return_value_index: Cell::new(0),
            return_values: VecDeque::new(),
            calls: Vec::new(),
            callback: None,
            call_base: false,
        }
    }

    pub fn add_return_value(&mut self, return_value: TReturnValue) {
        self.return_values.push_back(return_value);
    }

    pub fn add_return_values<const N: usize>(&mut self, return_values: [TReturnValue; N]) {
        self.return_values.extend(return_values.into_iter());
    }

    pub fn set_callback(&mut self, callback: impl FnMut() + 'static) {
        self.callback = Some(Arc::new(RefCell::new(callback)));
    }

    pub fn register_call(&mut self, call: Call) {
        self.calls.push(call);
    }

    pub fn check(&self, call: &Call) -> Vec<ArgCheckResult>
    where
        TArgsChecker: IDynArgsChecker,
    {
        self.args_checker.check(call.deref())
    }

    pub fn get_return_value(&self) -> Option<TReturnValue>
    where
        TReturnValue: Clone,
    {
        let current_return_value_index = self.current_return_value_index.get();
        let return_value = self.return_values.get(current_return_value_index).cloned();
        if return_value.is_some() {
            let new_current_return_value_index =
                (current_return_value_index + 1).min(self.return_values.len() - 1);
            self.current_return_value_index
                .set(new_current_return_value_index);
        }
        return return_value;
    }

    #[allow(private_bounds)]
    pub fn get_return_value_mut_ref(&self) -> Option<TReturnValue>
    where
        TReturnValue: IMutRefClone,
    {
        let current_return_value_index = self.current_return_value_index.get();
        let return_value = self
            .return_values
            .get(current_return_value_index)
            .map(IMutRefClone::mut_ref_clone);
        if return_value.is_some() {
            let new_current_return_value_index =
                (current_return_value_index + 1).min(self.return_values.len() - 1);
            self.current_return_value_index
                .set(new_current_return_value_index);
        }
        return return_value;
    }

    pub fn get_callback(&self) -> Option<Arc<RefCell<dyn FnMut()>>> {
        self.callback.clone()
    }
}

impl<TMock: IBaseCaller<TReturnValue>, TArgsChecker, TReturnValue>
    FnConfig<TMock, TArgsChecker, TReturnValue>
{
    pub fn set_call_base(&mut self) {
        self.call_base = true;
    }

    pub fn should_call_base(&self) -> bool {
        self.call_base
    }
}
