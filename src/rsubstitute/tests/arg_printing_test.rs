use rsubstitute::prelude::*;

#[mock]
trait Trait<'a> {
    fn accept_ref<'b>(&self, r: &'a &&'b i32) -> i32;

    fn accept_ref_ptr<'b>(&self, r: &'a &*const &&'b i32) -> i32;
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::record_panic;

    #[test]
    fn accept_ref_NoConfig_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&&5;

        // Act
        let panic_msg = record_panic(|| mock.accept_ref(r));

        // Assert
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait::accept_ref({r})"
        );
        assert_eq!(expected_panic_msg, panic_msg);
    }

    #[test]
    fn accept_ref_DidNotReceive_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&&5;
        let return_value = 175;
        let unexpected_r = &&&14;

        mock.setup.accept_ref(r).returns(return_value);

        // Act
        let actual_return_value = mock.accept_ref(r);

        // Assert
        assert_eq!(return_value, actual_return_value);

        let panic_msg = record_panic(|| mock.received.accept_ref(unexpected_r, Times::Once));
        let expected_panic_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::accept_ref((&&&i32): equal to {unexpected_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
Trait::accept_ref(*{r}*)
	1. r (&&&i32):
		Expected: {unexpected_r}
		Actual:   {r}"
        );
        assert_eq!(expected_panic_msg, panic_msg);
    }

    #[test]
    fn accept_ref_UnexpectedCall_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&&5;
        let return_value = 175;

        mock.setup.accept_ref(r).returns(return_value);

        // Act
        let actual_return_value = mock.accept_ref(r);

        // Assert
        assert_eq!(return_value, actual_return_value);

        let panic_msg = record_panic(|| mock.received.no_other_calls());
        let expected_panic_msg = format!(
            "Did not expect to receive any other calls. Received 1 unexpected call:
1. Trait::accept_ref({r})"
        );
        assert_eq!(expected_panic_msg, panic_msg);
    }

    #[test]
    fn accept_ref_ptr_NoConfig_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&(&&&5 as *const &&i32);

        // Act
        let panic_msg = record_panic(|| mock.accept_ref_ptr(r));

        // Assert
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait::accept_ref_ptr({r:?})"
        );
        assert_eq!(expected_panic_msg, panic_msg);
    }

    #[test]
    fn accept_ref_ptr_DidNotReceive_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&(&&&5 as *const &&i32);
        let return_value = 175;
        let unexpected_r = &&(&&&14 as *const &&i32);

        mock.setup.accept_ref_ptr(r).returns(return_value);

        // Act
        let actual_return_value = mock.accept_ref_ptr(r);

        // Assert
        assert_eq!(return_value, actual_return_value);

        let panic_msg = record_panic(|| mock.received.accept_ref_ptr(unexpected_r, Times::Once));
        let expected_panic_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::accept_ref_ptr((&&*const &&): equal to {unexpected_r:?})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
Trait::accept_ref_ptr(*{r:?}*)
	1. r (&&*const &&):
		Expected: {unexpected_r:?}
		Actual:   {r:?}"
        );
        assert_eq!(expected_panic_msg, panic_msg);
    }

    #[test]
    fn accept_ref_ptr_UnexpectedCall_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&(&&&5 as *const &&i32);
        let return_value = 175;

        mock.setup.accept_ref_ptr(r).returns(return_value);

        // Act
        let actual_return_value = mock.accept_ref_ptr(r);

        // Assert
        assert_eq!(return_value, actual_return_value);

        let panic_msg = record_panic(|| mock.received.no_other_calls());
        let expected_panic_msg = format!(
            "Did not expect to receive any other calls. Received 1 unexpected call:
1. Trait::accept_ref_ptr({r:?})"
        );
        assert_eq!(expected_panic_msg, panic_msg);
    }
}
