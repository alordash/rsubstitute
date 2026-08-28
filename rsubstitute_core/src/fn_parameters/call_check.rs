use crate::fn_parameters::DynCall;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::*;

mod formatting;
pub(crate) use formatting::*;

static CALL_ORDER_NUMBER: AtomicUsize = AtomicUsize::new(0);

pub struct CallCheck<'rs> {
    pub number: usize,
    verified: Cell<bool>,
    call: Rc<DynCall<'rs>>,
}

impl<'rs> CallCheck<'rs> {
    pub fn new(call: Rc<DynCall<'rs>>) -> Self {
        Self {
            number: CALL_ORDER_NUMBER.fetch_add(1, Ordering::AcqRel),
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
