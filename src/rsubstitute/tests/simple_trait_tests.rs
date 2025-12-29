#![allow(non_snake_case)]
use rsubstitute_proc_macro::mock;

#[mock]
trait Trait {
    fn f(&self);
}

#[mock]
#[allow(unused)]
trait AnotherTestTrait {}

// #[cfg(test)]
// #[allow(non_snake_case)]
// mod tests {
use rsubstitute::assertions::assert_panics;
use rsubstitute_core::Times;
use std::cell::RefCell;
use std::sync::Arc;

#[test]
fn f_Ok() {
    // Arrange
    let mock = TraitMock::new();
    let callback_flag = Arc::new(RefCell::new(false));
    let callback_flag_clone = callback_flag.clone();
    let return_value = ();
    mock.setup.f().returns_and_does(return_value, move || {
        *callback_flag_clone.borrow_mut() = true
    });

    // Act
    let result = mock.f();

    // Assert
    assert_eq!((), result);
    assert!(*callback_flag.borrow());
    mock.received.f(Times::Once);
}

#[test]
fn f_NoConfig_Ok() {
    // Arrange
    let mock = TraitMock::new();

    // Act
    let result = mock.f();

    // Assert
    assert_eq!((), result);
}

#[test]
fn f_MultipleTimes_Ok() {
    // Arrange
    let mock = TraitMock::new();

    // Act
    let result1 = mock.f();
    let result2 = mock.f();
    let result3 = mock.f();

    // Assert
    assert_eq!((), result1);
    assert_eq!((), result2);
    assert_eq!((), result3);

    mock.received.f(Times::Exactly(3));
}

#[test]
fn fn_MultipleTimes_OkPanics() {
    // Arrange
    let mock = TraitMock::new();

    // Act
    mock.f();
    mock.f();
    mock.f();

    // Assert
    assert_panics(
        || mock.received.f(Times::Once),
        r#"Expected to receive a call exactly once matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );

    assert_panics(
        || mock.received.f(Times::Exactly(1)),
        r#"Expected to receive a call exactly once matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );

    assert_panics(
        || mock.received.f(Times::Exactly(2)),
        r#"Expected to receive a call 2 times matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );

    assert_panics(
        || mock.received.f(Times::Exactly(4)),
        r#"Expected to receive a call 4 times matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );
}
