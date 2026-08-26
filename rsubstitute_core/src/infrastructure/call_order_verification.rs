use std::cell::Cell;

pub(crate) struct CallOrderState {
    pub perform_call_order_verification: Cell<bool>,
    pub last_call_order_number: Cell<usize>,
    // TODO - add last_call_fmt: Option<String>
}

thread_local! {
    static CALL_ORDER_STATE: CallOrderState = CallOrderState {
        perform_call_order_verification: Cell::new(false),
        last_call_order_number: Cell::new(0),
    };
}

pub(crate) fn enable() {
    CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(true))
}

pub(crate) fn disable() {
    CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.set(false))
}

pub(crate) fn should_perform() -> bool {
    CALL_ORDER_STATE.with(|x| x.perform_call_order_verification.get())
}

// Returns `Ok` if order is correct, `Err(last_call_order_number)` otherwise.
// TODO - instead of usize it should return last call fmt string
pub(crate) fn cmp_swap_call_order_number(new_call_order_number: usize) -> Result<(), usize> {
    CALL_ORDER_STATE.with(|x| {
        let last_call_order_number = x.last_call_order_number.get();
        if new_call_order_number <= last_call_order_number {
            return Err(last_call_order_number);
        }
        x.last_call_order_number.set(new_call_order_number);
        return Ok(());
    })
}

pub fn verify_call_order(verifications: impl Fn()) {
    enable();
    verifications();
    disable();
}