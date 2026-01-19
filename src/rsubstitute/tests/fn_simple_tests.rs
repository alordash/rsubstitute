#![allow(unused_variables)]
#![allow(non_snake_case)]

use rsubstitute::assertions::assert_panics;
use rsubstitute_core::Times;
use rsubstitute_proc_macro::mock;
use std::cell::RefCell;
use std::sync::Arc;

#[mock]
fn f() {}

#[test]
fn f_Ok() {
    // Arrange
    let callback_flag = Arc::new(RefCell::new(false));
    let callback_flag_clone = callback_flag.clone();
    f::setup().returns_and_does((), move || *callback_flag_clone.borrow_mut() = true);

    // Act
    let result = f();

    // Assert
    assert_eq!((), result);
    assert!(*callback_flag.borrow());
    f::received(Times::Once);
}

#[test]
fn f_NoConfig_Ok() {
    // Arrange
    f::setup();
    
    // Act
    let result = f();

    // Assert
    assert_eq!((), result);
    f::received(Times::Once);
}

#[test]
fn f_MultipleTime_Ok() {
    // Arrange
    f::setup();
    
    // Act
    let result1 = f();
    let result2 = f();
    let result3 = f();

    // Assert
    assert_eq!((), result1);
    assert_eq!((), result2);
    assert_eq!((), result3);

    f::received(Times::Exactly(3));
}

#[test]
fn f_MultipleTimes_PanicsOk() {
    // Arrange
    f::setup();
    
    // Act
    f();
    f();
    f();

    // Assert
    assert_panics(
        || f::received(Times::Once),
        r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
    );

    assert_panics(
        || f::received(Times::Exactly(1)),
        r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
    );

    assert_panics(
        || f::received(Times::Exactly(2)),
        r#"Expected to receive a call 2 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
    );

    assert_panics(
        || f::received(Times::Exactly(4)),
        r#"Expected to receive a call 4 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
    );
}
