use std::panic;

fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

pub fn assert_panics<T>(callback: impl FnOnce() -> T, expected_error_message: impl AsRef<str>) {
    let panic_error = catch_unwind_silent(panic::AssertUnwindSafe(callback));
    if let Some(actual_error) = panic_error.err().as_deref() {
        let actual_error_message = actual_error.downcast_ref::<String>().unwrap();
        let expected_error_message_str = expected_error_message.as_ref();
        if expected_error_message_str != actual_error_message {
            panic!(
                r#"Wrong panic message.
Expected: {expected_error_message_str:?}
  Actual: {actual_error_message:?}"#
            );
        }
        assert_eq!(expected_error_message.as_ref(), actual_error_message);
    } else {
        panic!(
            r#"Expected to panic with following error message:
{:?}"#,
            expected_error_message.as_ref()
        );
    }
}
