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
use __rsubstitute_generated_Trait::*;
use rsubstitute::assertions::assert_panics;
use rsubstitute_core::Times;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn f_Ok() {
    // Arrange
    let mock = TraitMock::new();
    let callback_flag = Rc::new(RefCell::new(false));
    let callback_flag_clone = callback_flag.clone();
    let return_value = ();
    mock.f().returns_and_does(return_value, move || {
        *callback_flag_clone.borrow_mut() = true
    });

    // Act
    let result = Trait::f(&mock);

    // Assert
    assert_eq!((), result);
    assert!(*callback_flag.borrow());
    mock.received_f(Times::Once);
}

#[test]
fn f_NoConfig_Ok() {
    // Arrange
    let mock = TraitMock::new();

    // Act
    let result = Trait::f(&mock);

    // Assert
    assert_eq!((), result);
}

#[test]
fn f_MultipleTimes_Ok() {
    // Arrange
    let mock = TraitMock::new();

    // Act
    let result1 = Trait::f(&mock);
    let result2 = Trait::f(&mock);
    let result3 = Trait::f(&mock);

    // Assert
    assert_eq!((), result1);
    assert_eq!((), result2);
    assert_eq!((), result3);

    mock.received_f(Times::Exactly(3))
        .received_f(Times::AtLeast(0))
        .received_f(Times::AtLeast(1))
        .received_f(Times::AtLeast(2))
        .received_f(Times::AtLeast(3))
        .received_f(Times::AtMost(3))
        .received_f(Times::AtMost(4));
}

#[test]
fn fn_MultipleTimes_OkPanics() {
    // Arrange
    let mock = TraitMock::new();

    // Act
    Trait::f(&mock);
    Trait::f(&mock);
    Trait::f(&mock);

    // Assert
    assert_panics(
        || mock.received_f(Times::Once),
        r#"Expected to receive a call exactly once matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );

    assert_panics(
        || mock.received_f(Times::Exactly(1)),
        r#"Expected to receive a call exactly once matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );

    assert_panics(
        || mock.received_f(Times::Exactly(2)),
        r#"Expected to receive a call 2 times matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );

    assert_panics(
        || mock.received_f(Times::Exactly(4)),
        r#"Expected to receive a call 4 times matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );

    assert_panics(
        || mock.received_f(Times::AtLeast(4)),
        r#"Expected to receive a call at least 4 times matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );

    assert_panics(
        || mock.received_f(Times::AtMost(2)),
        r#"Expected to receive a call at most 2 times matching:
	f_data()
Actually received 3 matching calls:
	f_data()
	f_data()
	f_data()
Received no non-matching calls"#,
    );
}
