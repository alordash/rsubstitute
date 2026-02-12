use crate::args_matching::{ArgCheckResult, ArgsChecker, IArgsChecker};
use crate::fn_parameters::Call;
use crate::{IBaseCaller, ReturnValue};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct FnConfig<'a, TMock> {
    _phantom_mock: PhantomData<TMock>,
    args_checker: ArgsChecker<'a>,
    current_return_value_index: Cell<usize>,
    return_values: VecDeque<ReturnValue<'a>>,
    calls: Vec<Call<'a>>,
    callback: Option<Arc<RefCell<dyn FnMut()>>>,
    call_base: bool,
}

impl<'a, TMock> FnConfig<'a, TMock> {
    pub(crate) fn new(args_checker: ArgsChecker<'a>) -> Self {
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

    pub(crate) fn add_return_value(&mut self, return_value: ReturnValue<'a>) {
        self.return_values.push_back(return_value);
    }

    pub(crate) fn add_return_values<const N: usize>(
        &mut self,
        return_values: [ReturnValue<'a>; N],
    ) {
        self.return_values.extend(return_values.into_iter());
    }

    pub(crate) fn set_callback(&mut self, callback: impl FnMut() + 'static) {
        self.callback = Some(Arc::new(RefCell::new(callback)));
    }

    pub(crate) fn register_call(&mut self, call: Call<'a>) {
        self.calls.push(call);
    }

    pub(crate) fn check(&self, call: &Call) -> Vec<ArgCheckResult> {
        self.args_checker.check(&call)
    }

    pub(crate) fn get_return_value(&self) -> Option<ReturnValue<'a>> {
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

    pub(crate) fn get_callback(&self) -> Option<Arc<RefCell<dyn FnMut()>>> {
        self.callback.clone()
    }
}

impl<'a, TMock: IBaseCaller> FnConfig<'a, TMock> {
    pub(crate) fn set_call_base(&mut self) {
        self.call_base = true;
    }

    pub(crate) fn should_call_base(&self) -> bool {
        self.call_base
    }
}
