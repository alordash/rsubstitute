use rsubstitute::*;
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
struct Payload<'a> {
    value: i32,
    reference: &'a i32,
}

impl<'a> PartialEq for Payload<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<'a> Deref for Payload<'a> {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.reference
    }
}

#[mock]
fn work<'a>(_: Payload<'a>) {}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::assert_panics;

    #[test]
    fn ArgEq_ComparesUsingPartialEq() {
        // Arrange
        let reference = &1;
        let first = Payload {
            value: 10,
            reference,
        };
        let second = Payload {
            value: 20,
            reference,
        };

        // Act
        work(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        work::received(Arg::eq(first), Times::Once)
            .received(Arg::eq(second), Times::Never)
            .no_other_calls();
    }

    #[test]
    fn ArgEq_WhenPartialEqReturnsFalse_Panics() {
        // Arrange
        let reference = &1;
        let first = Payload {
            value: 10,
            reference,
        };
        let second = Payload {
            value: 20,
            reference,
        };

        // Act
        work(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        assert_panics(
            || work::received(Arg::eq(second), Times::Once),
            "Expected to receive a call exactly once matching:
	work((arg_tests::Payload<'_>): equal to Payload { value: 20, reference: 1 })
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
work(*Payload { value: 10, reference: 1 }*)
	1. __arg0 (arg_tests::Payload<'_>):
		Expected: Payload { value: 20, reference: 1 }
		Actual:   Payload { value: 10, reference: 1 }",
        );
    }

    #[test]
    fn ArgRefEq_ComparesUsingDeref() {
        // Arrange
        let reference = &1;
        let first = Payload {
            value: 10,
            reference,
        };
        let second = Payload {
            value: 20,
            reference,
        };

        // Act
        work(first);

        // Assert
        assert!(core::ptr::eq(first.reference, second.reference));
        work::received(Arg::ref_eq(first), Times::Once)
            .received(Arg::ref_eq(second), Times::Once)
            .no_other_calls();
    }

    #[test]
    fn ArgRefEq_WhenDerefReturnsDifferentRef_Panics() {
        // Arrange
        let first_reference = &core::hint::black_box(1);
        let first = Payload {
            value: 10,
            reference: first_reference,
        };
        let second_reference = &core::hint::black_box(1);
        let second = Payload {
            value: 20,
            reference: second_reference,
        };

        // Act
        work(first);

        // Assert
        assert!(!core::ptr::eq(first_reference, second_reference));
        let first_ptr = first_reference as *const _;
        let second_ptr = second_reference as *const _;
        assert_panics(
            || work::received(Arg::ref_eq(second), Times::Once),
            format!(
                "Expected to receive a call exactly once matching:
	work((arg_tests::Payload<'_>): equal to Payload {{ value: 20, reference: 1 }})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
work(*Payload {{ value: 10, reference: 1 }}*)
	1. __arg0 (arg_tests::Payload<'_>):
		Expected (ptr: {second_ptr:?}): Payload {{ value: 20, reference: 1 }}
		Actual   (ptr: {first_ptr:?}): Payload {{ value: 10, reference: 1 }}"
            ),
        );
    }
}
