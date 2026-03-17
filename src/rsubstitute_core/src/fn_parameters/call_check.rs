use std::cell::Cell;
use std::sync::Arc;
use crate::fn_parameters::DynCall;

pub struct CallCheck<'rs> {
    pub number: usize,
    verified: Cell<bool>,
    call: Arc<DynCall<'rs>>,
}

impl<'rs> CallCheck<'rs> {
    pub fn new(number: usize, call: Arc<DynCall<'rs>>) -> Self {
        Self {
            number,
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
