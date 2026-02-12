use crate::args_matching::{ArgCheckResult, IDynArgsChecker};
use crate::fn_parameters::Call;
use crate::{IBaseCaller, ReturnValue};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

pub struct FnConfig<TMock, TArgsChecker> {
    _phantom_mock: PhantomData<TMock>,
    args_checker: TArgsChecker,
    current_return_value_index: Cell<usize>,
    return_values: VecDeque<ReturnValue>,
    calls: Vec<Call>,
    callback: Option<Arc<RefCell<dyn FnMut()>>>,
    call_base: bool,
}

impl<TMock, TArgsChecker> FnConfig<TMock, TArgsChecker> {
    pub(crate) fn new(args_checker: TArgsChecker) -> Self {
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

    pub(crate) fn add_return_value(&mut self, return_value: ReturnValue) {
        self.return_values.push_back(return_value);
    }

    pub(crate) fn add_return_values<const N: usize>(&mut self, return_values: [ReturnValue; N]) {
        self.return_values.extend(return_values.into_iter());
    }

    pub(crate) fn set_callback(&mut self, callback: impl FnMut() + 'static) {
        self.callback = Some(Arc::new(RefCell::new(callback)));
    }

    pub(crate) fn register_call(&mut self, call: Call) {
        self.calls.push(call);
    }

    pub(crate) fn check(&self, call: &Call) -> Vec<ArgCheckResult>
    where
        TArgsChecker: IDynArgsChecker,
    {
        self.args_checker.check(call.deref())
    }

    pub(crate) fn get_return_value(&self) -> Option<ReturnValue> {
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

    // #[allow(private_bounds)]
    // pub(crate) fn get_return_value_mut_ref(&self) -> Option<ReturnValue>
    // {
    //     let current_return_value_index = self.current_return_value_index.get();
    //     let return_value = self
    //         .return_values
    //         .get(current_return_value_index)
    //         .map(IMutRefClone::mut_ref_clone);
    //     if return_value.is_some() {
    //         let new_current_return_value_index =
    //             (current_return_value_index + 1).min(self.return_values.len() - 1);
    //         self.current_return_value_index
    //             .set(new_current_return_value_index);
    //     }
    //     return return_value;
    // }

    pub(crate) fn get_callback(&self) -> Option<Arc<RefCell<dyn FnMut()>>> {
        self.callback.clone()
    }
}

impl<TMock: IBaseCaller, TArgsChecker> FnConfig<TMock, TArgsChecker> {
    pub(crate) fn set_call_base(&mut self) {
        self.call_base = true;
    }

    pub(crate) fn should_call_base(&self) -> bool {
        self.call_base
    }
}
