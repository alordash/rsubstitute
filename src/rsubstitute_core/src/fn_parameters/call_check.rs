use std::cell::Cell;
use std::sync::Arc;
use crate::fn_parameters::DynCall;

pub(crate) struct CallCheck<'rs> {
    verified: Cell<bool>,
    call: Arc<DynCall<'rs>>,
}

impl<'rs> CallCheck<'rs> {
    pub fn new(call: Arc<DynCall<'rs>>) -> Self {
        Self {
            verified: Cell::new(false),
            call,
        }
    }

    pub fn mark_as_verified(&self) {
        self.verified.set(true);
    }

    pub fn is_not_verified(&self) -> bool {
        !self.verified.get()
    }

    pub fn get_call(&self) -> &DynCall<'rs> {
        &self.call
    }
}
