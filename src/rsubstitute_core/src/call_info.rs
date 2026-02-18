use std::cell::Cell;
use std::sync::Arc;

pub(crate) struct CallCheck<TCall> {
    verified: Cell<bool>,
    call: Arc<TCall>,
}

impl<TCall> CallCheck<TCall> {
    pub fn new(call: Arc<TCall>) -> Self {
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

    pub fn get_call(&self) -> &TCall {
        &self.call
    }
}
