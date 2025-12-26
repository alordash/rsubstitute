#![allow(non_snake_case)]
use rsubstitute::assertions::assert_panics;
use rsubstitute_core::Times;
use rsubstitute_core::args_matching::Arg;
use rsubstitute_proc_macro::mock;

#[mock]
trait Trait {
    fn accept_value(&self, v: i32);

    fn return_value(&self) -> i32;

    fn accept_value_return_value(&self, v: i32) -> f32;

    fn accept_two_values(&self, v1: i32, v2: f32);

    fn accept_two_values_return_value(&self, v1: i32, v2: f32) -> String;
}

#[test]
fn accept_value_Ok() {
    // Arrange
    let mock = TraitMock::new();
    let first_value = 10;
    let second_value = 22;

    // Act
    mock.accept_value(first_value);
    mock.accept_value(second_value);
    mock.accept_value(second_value);

    // Assert
    mock.received
        .accept_value(Arg::Any, Times::Exactly(3))
        .accept_value(Arg::Eq(first_value), Times::Once)
        .accept_value(
            Arg::Is(|actual_value| actual_value == first_value),
            Times::Once,
        )
        .accept_value(Arg::Eq(second_value), Times::Exactly(2))
        .accept_value(
            Arg::Is(|actual_value| actual_value == second_value),
            Times::Exactly(2),
        );
}

#[test]
fn accept_value_ArgAny_OkPanics() {
    // Arrange
    let mock = TraitMock::new();
    let first_value = 10;
    let second_value = 22;

    // Act
    mock.accept_value(first_value);
    mock.accept_value(second_value);

    // Assert
    assert_panics(
        || mock.received.accept_value(Arg::Any, Times::Never),
        format!(
            r#"Expected to never receive a call matching:
	accept_value_data((i32): any)
Actually received 2 matching calls:
	accept_value_data({first_value})
	accept_value_data({second_value})
Received no non-matching calls"#
        ),
    );
    assert_panics(
        || mock.received.accept_value(Arg::Any, Times::Once),
        format!(
            r#"Expected to receive a call exactly once matching:
	accept_value_data((i32): any)
Actually received 2 matching calls:
	accept_value_data({first_value})
	accept_value_data({second_value})
Received no non-matching calls"#
        ),
    );
    assert_panics(
        || mock.received.accept_value(Arg::Any, Times::Exactly(3)),
        format!(
            r#"Expected to receive a call 3 times matching:
	accept_value_data((i32): any)
Actually received 2 matching calls:
	accept_value_data({first_value})
	accept_value_data({second_value})
Received no non-matching calls"#
        ),
    );
}

#[test]
fn accept_value_ArgEq_OkPanics() {
    // Arrange
    let mock = TraitMock::new();
    let first_value = 10;
    let second_value = 22;

    // Act
    mock.accept_value(first_value);
    mock.accept_value(second_value);

    // Assert
    assert_panics(
        || {
            mock.received
                .accept_value(Arg::Eq(first_value), Times::Never)
        },
        format!(
            r#"Expected to never receive a call matching:
	accept_value_data((i32): equal to {first_value})
Actually received 1 matching call:
	accept_value_data({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value_data(*{second_value}*)
	1. v (i32):
		Expected: {first_value}
		Actual:   {second_value}"#
        ),
    );
    assert_panics(
        || {
            mock.received
                .accept_value(Arg::Eq(first_value), Times::Exactly(3))
        },
        format!(
            r#"Expected to receive a call 3 times matching:
	accept_value_data((i32): equal to {first_value})
Actually received 1 matching call:
	accept_value_data({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value_data(*{second_value}*)
	1. v (i32):
		Expected: {first_value}
		Actual:   {second_value}"#
        ),
    );
    assert_panics(
        || {
            mock.received
                .accept_value(Arg::Eq(second_value), Times::Never)
        },
        format!(
            r#"Expected to never receive a call matching:
	accept_value_data((i32): equal to {second_value})
Actually received 1 matching call:
	accept_value_data({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value_data(*{first_value}*)
	1. v (i32):
		Expected: {second_value}
		Actual:   {first_value}"#
        ),
    );
    assert_panics(
        || {
            mock.received
                .accept_value(Arg::Eq(second_value), Times::Exactly(3))
        },
        format!(
            r#"Expected to receive a call 3 times matching:
	accept_value_data((i32): equal to {second_value})
Actually received 1 matching call:
	accept_value_data({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value_data(*{first_value}*)
	1. v (i32):
		Expected: {second_value}
		Actual:   {first_value}"#
        ),
    );
}

#[test]
fn accept_value_ArgIs_OkPanics() {
    // Arrange
    let mock = TraitMock::new();
    let first_value = 10;
    let second_value = 22;

    // Act
    mock.accept_value(first_value);
    mock.accept_value(second_value);

    // Assert
    assert_panics(
        || {
            mock.received.accept_value(
                Arg::Is(|actual_value| actual_value == first_value),
                Times::Never,
            )
        },
        format!(
            r#"Expected to never receive a call matching:
	accept_value_data((i32): custom predicate)
Actually received 1 matching call:
	accept_value_data({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value_data(*{second_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {second_value}"#
        ),
    );
    assert_panics(
        || {
            mock.received.accept_value(
                Arg::Is(|actual_value| actual_value == first_value),
                Times::Exactly(3),
            )
        },
        format!(
            r#"Expected to receive a call 3 times matching:
	accept_value_data((i32): custom predicate)
Actually received 1 matching call:
	accept_value_data({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value_data(*{second_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {second_value}"#
        ),
    );
    assert_panics(
        || {
            mock.received.accept_value(
                Arg::Is(|actual_value| actual_value == second_value),
                Times::Never,
            )
        },
        format!(
            r#"Expected to never receive a call matching:
	accept_value_data((i32): custom predicate)
Actually received 1 matching call:
	accept_value_data({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value_data(*{first_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {first_value}"#
        ),
    );
    assert_panics(
        || {
            mock.received.accept_value(
                Arg::Is(|actual_value| actual_value == second_value),
                Times::Exactly(3),
            )
        },
        format!(
            r#"Expected to receive a call 3 times matching:
	accept_value_data((i32): custom predicate)
Actually received 1 matching call:
	accept_value_data({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value_data(*{first_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {first_value}"#
        ),
    );
}
