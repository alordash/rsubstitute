use rsubstitute_proc_macro::mock;

#[mock]
trait Trait<'a, T1, const B: bool> {
    fn work<T2, 'b, const N: usize>(&self, v: &'b T1) -> T2;
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute_core::Times;

    #[test]
    fn work_NoConfigs_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 5>(&14));

        // Assert
        let expected_panic_msg = "Mock wasn't configured to handle following call:
	Trait::work<f32, 5>(14)";
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_OnlyUnsuitableConfigs_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        mock.setup.work::<f32, 1>(&2).returns(3.0f32);
        mock.setup.work::<f32, 10>(&value).returns(3.0f32);
        mock.setup.work::<[u8; 4], 10>(&value).returns([1, 2, 3, 4]);
        mock.setup.work::<[u8; 4], 10>(&2).returns([1, 2, 3, 4]);

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 1>(&value));

        // Assert
        let expected_panic_msg = "Mock wasn't configured to handle following call:
	Trait::work<f32, 1>(5)
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: Trait::work(*5*)";
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_NoReturnValue_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup.work::<f32, 1>(&value);
        mock.setup.work::<f32, 1>(&value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);
        mock.received
            .work::<f32, 1>(&value, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn work_DidNotReceiveSameGenerics_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let actual_value = 5;
        let expected_value = actual_value + 1;
        let returned_value = 3.0f32;
        const N: usize = 1;
        mock.setup
            .work::<f32, N>(&actual_value)
            .returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&actual_value);
        let panic_msg = record_panic(|| mock.received.work::<f32, 1>(&expected_value, Times::Once));

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let actual_value_ptr = core::ptr::from_ref(&actual_value);
        let expected_value_ptr = core::ptr::from_ref(&expected_value);
        let expected_panic_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::work<f32, {N}>((&i32): equal to {expected_value})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
Trait::work(*5*)
	1. v (&i32):
		Expected reference (ptr: {expected_value_ptr:?}): 6
		Actual reference   (ptr: {actual_value_ptr:?}): 5"
        );

        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn work_DidNotReceiveDifferentGenerics_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup.work::<f32, 1>(&value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&value);
        let panic_msg = record_panic(|| mock.received.work::<String, 124>(&value, Times::Once));

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let expected_panic_msg = "Expected to receive a call exactly once matching:
	Trait::work<alloc::string::String, 124>((&i32): equal to 5)
Actually received no matching calls
Received no non-matching calls";
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_ReceivedUnexpectedCalls_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let first_value = 5;
        let first_returned_value = 3.0f32;
        const FIRST_N: usize = 1;
        let second_value = 100;
        let second_returned_value = [4; 3];
        const SECOND_N: usize = 200;
        mock.setup
            .work::<f32, FIRST_N>(&first_value)
            .returns(first_returned_value);
        mock.setup
            .work::<_, SECOND_N>(&second_value)
            .returns(second_returned_value);

        // Act
        let actual_first_returned_value = mock.work::<f32, FIRST_N>(&first_value);
        let actual_second_returned_value = mock.work::<[i32; 3], SECOND_N>(&second_value);
        let panic_msg = record_panic(|| mock.received.no_other_calls());

        // Assert
        assert_eq!(first_returned_value, actual_first_returned_value);
        assert_eq!(second_returned_value, actual_second_returned_value);

        let expected_panic_msg =
            "Did not expect to receive any other calls. Received 2 unexpected calls:
1. Trait::work<f32, 1>(5)
2. Trait::work<[i32; 3], 200>(100)";
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }
}
