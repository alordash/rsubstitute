use crate::fn_parameters::Call;
use std::cell::Cell;
use std::sync::Arc;

pub(crate) struct CallInfo<'a> {
    verified: Cell<bool>,
    call: Arc<Call<'a>>,
}

impl<'a> CallInfo<'a> {
    pub fn new(call: Arc<Call<'a>>) -> Self {
        Self {
            verified: Cell::new(false),
            call,
        }
    }

    pub fn mark_as_verified(&'a self) {
        self.verified.set(true);
    }

    pub fn is_not_verified(&self) -> bool {
        !self.verified.get()
    }

    pub fn get_call(&'_ self) -> &Call<'a> {
        &self.call
    }
}
