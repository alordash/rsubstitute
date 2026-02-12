use crate::IBaseCaller;
use crate::args_matching::{ArgCheckResult, IArgsChecker};
use crate::i_mut_ref_clone::IMutRefClone;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct FnConfig<TMock, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue> {
    _phantom_mock: PhantomData<TMock>,
    args_checker: TArgsChecker,
    current_return_value_index: usize,
    return_values: VecDeque<TReturnValue>,
    calls: Vec<TCall>,
    callback: Option<Arc<RefCell<dyn FnMut()>>>,
    call_base: bool,
}

impl<TMock, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue>
    FnConfig<TMock, TCall, TArgsChecker, TReturnValue>
{
    pub fn new(args_checker: TArgsChecker) -> Self {
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

    pub fn add_return_value(&mut self, return_value: TReturnValue) {
        self.return_values.push_back(return_value);
    }

    pub fn add_return_values<const N: usize>(&mut self, return_values: [TReturnValue; N]) {
        self.return_values.extend(return_values.into_iter());
    }

    pub fn set_callback(&mut self, callback: impl FnMut() + 'static) {
        self.callback = Some(Arc::new(RefCell::new(callback)));
    }

    pub fn register_call(&mut self, call: TCall) {
        self.calls.push(call);
    }

    pub fn check(&'_ self, call: &TCall) -> Vec<ArgCheckResult> {
        self.args_checker.check(call)
    }

    pub fn get_return_value(&mut self) -> Option<TReturnValue>
    where
        TReturnValue: Clone,
    {
        let return_value = self
            .return_values
            .get(self.current_return_value_index)
            .cloned();
        if return_value.is_some() {
            self.current_return_value_index =
                (self.current_return_value_index + 1).min(self.return_values.len() - 1);
        }
        return return_value;
    }

    pub fn get_return_value_mut_ref(&mut self) -> Option<TReturnValue>
    where
        TReturnValue: IMutRefClone,
    {
        let return_value = self
            .return_values
            .get(self.current_return_value_index)
            .map(|x| x.mut_ref_clone());
        if return_value.is_some() {
            self.current_return_value_index =
                (self.current_return_value_index + 1).min(self.return_values.len() - 1);
        }
        return return_value;
    }

    pub fn get_callback(&self) -> Option<Arc<RefCell<dyn FnMut()>>> {
        self.callback.clone()
    }
}

impl<
    TMock: IBaseCaller<TCall, TReturnValue>,
    TCall,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue,
> FnConfig<TMock, TCall, TArgsChecker, TReturnValue>
{
    pub fn set_call_base(&mut self) {
        self.call_base = true;
    }

    pub fn should_call_base(&self) -> bool {
        self.call_base
    }
}
