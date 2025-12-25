use std::panic;

fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

pub fn assert_panics<T>(callback: impl FnOnce() -> T, expected_error_message: &'static str) {
    let panic_error = catch_unwind_silent(panic::AssertUnwindSafe(callback));
    if let Some(actual_error) = panic_error.err().as_deref() {
        let actual_error_message = actual_error.downcast_ref::<String>().unwrap();
        assert_eq!(expected_error_message, actual_error_message);
    } else {
        panic!("Expected to panic!");
    }
}
