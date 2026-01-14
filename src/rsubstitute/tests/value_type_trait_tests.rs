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

mod accept_value {
    use super::*;

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
	accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
Received no non-matching calls"#
            ),
        );
        assert_panics(
            || mock.received.accept_value(Arg::Any, Times::Once),
            format!(
                r#"Expected to receive a call exactly once matching:
	accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
Received no non-matching calls"#
            ),
        );
        assert_panics(
            || mock.received.accept_value(Arg::Any, Times::Exactly(3)),
            format!(
                r#"Expected to receive a call 3 times matching:
	accept_value((i32): any)
Actually received 2 matching calls:
	accept_value({first_value})
	accept_value({second_value})
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
	accept_value((i32): equal to {first_value})
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
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
	accept_value((i32): equal to {first_value})
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
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
	accept_value((i32): equal to {second_value})
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
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
	accept_value((i32): equal to {second_value})
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
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
	accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
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
	accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({first_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{second_value}*)
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
	accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
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
	accept_value((i32): custom predicate)
Actually received 1 matching call:
	accept_value({second_value})
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_value(*{first_value}*)
	1. v (i32):
		Custom predicate didn't match passed value. Received value: {first_value}"#
            ),
        );
    }
}

mod return_value {
    use super::*;
    use std::cell::RefCell;
    use std::sync::Arc;

    #[test]
    fn return_value_Single_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let value = 10;
        mock.setup.return_value().returns(value);

        // Act
        let actual_value = mock.return_value();

        // Assert
        assert_eq!(value, actual_value);
    }

    #[test]
    fn return_value_UsesLastConfiguration_Ok() {
        // Arrange
        #[derive(Debug, PartialEq)]
        enum Result {
            DidNotChange,
            SecondConfigChanged,
            ThirdConfigChanged,
        }

        let mock = TraitMock::new();
        let first_value = 10;
        let second_value = 22;
        let third_value = 333;
        let callback_result = Arc::new(RefCell::new(Result::DidNotChange));
        let first_callback_counter_clone = callback_result.clone();
        let second_callback_counter_clone = callback_result.clone();
        mock.setup
            .return_value()
            .returns(first_value)
            .return_value()
            .returns_and_does(second_value, move || {
                *first_callback_counter_clone.borrow_mut() = Result::SecondConfigChanged
            })
            .return_value()
            .returns_and_does(third_value, move || {
                *second_callback_counter_clone.borrow_mut() = Result::ThirdConfigChanged
            });

        // Act
        let actual_first_value = mock.return_value();
        let actual_second_value = mock.return_value();
        let actual_third_value = mock.return_value();

        // Assert
        assert_eq!(third_value, actual_first_value);
        assert_eq!(third_value, actual_second_value);
        assert_eq!(third_value, actual_third_value);
        assert_eq!(Result::ThirdConfigChanged, *callback_result.borrow());
    }

    #[test]
    fn return_value_Many_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let first_value = 10;
        let second_value = 22;
        let third_value = 333;
        mock.setup
            .return_value()
            .returns_many(&[first_value, second_value, third_value]);

        // Act
        let actual_first_value = mock.return_value();
        let actual_second_value = mock.return_value();
        let actual_third_value = mock.return_value();
        let actual_fourth_value = mock.return_value();

        // Assert
        assert_eq!(first_value, actual_first_value);
        assert_eq!(second_value, actual_second_value);
        assert_eq!(third_value, actual_third_value);
        assert_eq!(third_value, actual_fourth_value);
    }

    #[test]
    fn return_value_ManyToSingle_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let second_value = 22;
        mock.setup
            .return_value()
            .returns_many(&[1, 2, 3])
            .return_value()
            .returns(second_value);

        // Act
        let actual_first_value = mock.return_value();
        let actual_second_value = mock.return_value();

        // Assert
        assert_eq!(second_value, actual_first_value);
        assert_eq!(second_value, actual_second_value);
    }

    #[test]
    fn return_value_ManyWithCallback_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let callback_counter = Arc::new(RefCell::new(0));
        let callback_counter_clone = callback_counter.clone();
        let first_value = 10;
        let second_value = 22;
        mock.setup
            .return_value()
            .returns_many_and_does(&[first_value, second_value], move || {
                *callback_counter_clone.borrow_mut() += 1
            });

        // Act
        let actual_first_value = mock.return_value();
        let actual_second_value = mock.return_value();
        let actual_third_value = mock.return_value();

        // Assert
        assert_eq!(3, *callback_counter.borrow());

        assert_eq!(first_value, actual_first_value);
        assert_eq!(second_value, actual_second_value);
        assert_eq!(second_value, actual_third_value);
    }
}
