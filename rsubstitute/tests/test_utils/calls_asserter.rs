use std::sync::atomic::{AtomicBool, Ordering};

// Helper class for callbacks verification.
// Leaking for ability to `Copy` so no need to create clones for moving them in closures.
#[derive(Copy, Clone)]
pub struct CallsAsserter {
    first_called: &'static AtomicBool,
    second_called: &'static AtomicBool,
}
// For some reason rust compiler thinks that `second` (and some other functions) are not used
// anywhere, even though they're used in tests.
#[allow(unused)]
impl CallsAsserter {
    pub fn new() -> Self {
        Self {
            first_called: Box::leak(Box::new(AtomicBool::new(false))),
            second_called: Box::leak(Box::new(AtomicBool::new(false))),
        }
    }
    pub fn first(&self) {
        self.first_called.store(true, Ordering::Relaxed);
    }
    pub fn second(&self) {
        self.second_called.store(true, Ordering::Relaxed);
    }

    pub fn assert_none_was_called(&self) {
        match (
            self.first_called.load(Ordering::Relaxed),
            self.second_called.load(Ordering::Relaxed),
        ) {
            (false, false) => (),
            (true, false) => panic!(
                "Calls assertions failed: expected neither first nor second method to be called, actually first method was called."
            ),
            (false, true) => panic!(
                "Calls assertions failed: expected neither first nor second method to be called, actually second method was called."
            ),
            (true, true) => panic!(
                "Calls assertions failed: expected neither first nor second method to be called, actually both first and second methods were called."
            ),
        }
    }

    pub fn assert_only_first_was_called(&self) {
        match (
            self.first_called.load(Ordering::Relaxed),
            self.second_called.load(Ordering::Relaxed),
        ) {
            (false, false) => panic!(
                "Calls assertions failed: expected only first method to be called, actually nor first nor second method was called."
            ),
            (true, false) => (),
            (false, true) => panic!(
                "Calls assertions failed: expected only first method to be called, actually only second method was called."
            ),
            (true, true) => panic!(
                "Calls assertion failed: expected only first method to be called, actually both first and second methods were called."
            ),
        }
    }

    pub fn assert_only_second_was_called(&self) {
        match (
            self.first_called.load(Ordering::Relaxed),
            self.second_called.load(Ordering::Relaxed),
        ) {
            (false, false) => panic!(
                "Calls assertions failed: expected only second method to be called, actually nor first nor second method was called."
            ),
            (true, false) => panic!(
                "Calls assertions failed: expected only second method to be called, actually only first method was called."
            ),
            (false, true) => (),
            (true, true) => panic!(
                "Calls assertion failed: expected only second method to be called, actually both first and second methods were called."
            ),
        }
    }

    pub fn assert_both_were_called(&self) {
        match (
            self.first_called.load(Ordering::Relaxed),
            self.second_called.load(Ordering::Relaxed),
        ) {
            (false, false) => panic!(
                "Calls assertions failed: expected both first and second method to be called, actually nor first nor second method was called."
            ),
            (true, false) => panic!(
                "Calls assertions failed: expected both first and second method to be called, actually only first method was called."
            ),
            (false, true) => panic!(
                "Calls assertions failed: expected both first and second method to be called, actually only second method was called."
            ),
            (true, true) => (),
        }
    }
}
