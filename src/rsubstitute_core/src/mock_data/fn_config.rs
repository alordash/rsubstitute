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
    current_return_value_index: usize,
    return_values: VecDeque<DynReturnValue<'rs>>,
    calls: Vec<Arc<DynCall<'rs>>>,
    callback: Option<Arc<RefCell<dyn FnMut()>>>,
    call_base: bool,
}

impl<'rs, TMock> FnConfig<'rs, TMock> {
    pub(crate) fn new(args_checker: DynArgsChecker<'rs>) -> Self {
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

    pub(crate) fn add_return_value(&mut self, return_value: DynReturnValue<'rs>) {
        self.return_values.push_back(return_value);
    }

    pub(crate) fn add_return_values<const N: usize>(
        &mut self,
        return_values: [DynReturnValue<'rs>; N],
    ) {
        self.return_values.extend(return_values.into_iter());
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
        let return_value = self
            .return_values
            .get(self.current_return_value_index)
            .map(|dyn_return_value| dyn_return_value.clone());
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
