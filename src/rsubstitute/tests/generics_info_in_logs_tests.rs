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
        panic!("{panic_msg}");
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
        panic!("{panic_msg}")
    }

    #[test]
    fn work_NoReturnValue_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        mock.setup.work::<f32, 1>(&value);
        mock.setup.work::<f32, 1>(&value).returns(3.0f32);

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 1>(&value));

        // Assert
        panic!("{panic_msg}")
    }

    #[test]
    fn work_DidNotReceiveSameGenerics_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup.work::<f32, 1>(&value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let panic_msg = record_panic(|| mock.received.work::<f32, 1>(&(value + 1), Times::Once));

        panic!("{panic_msg}")
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

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let panic_msg = record_panic(|| mock.received.work::<String, 124>(&value, Times::Once));

        panic!("{panic_msg}")
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

        // Assert
        assert_eq!(first_returned_value, actual_first_returned_value);
        assert_eq!(second_returned_value, actual_second_returned_value);

        let panic_msg = record_panic(|| mock.received.no_other_calls());

        panic!("{panic_msg}")
    }
}
