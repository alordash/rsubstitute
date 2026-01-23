use rsubstitute::macros::mock;

#[mock]
trait Trait<T1, T2> {
    fn work(&self, value: T1) -> T1;

    fn rest(&self, value: T1) -> T2;
}

mod trait_generic_tests {
    #![allow(unused_variables)]
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    #[test]
    fn work_first_type_i32_Ok() {
        // Arrange
        let mock = TraitMock::<_, String>::new();
        let accepted_value = 10;
        let returned_value = 200;
        mock.setup.work(accepted_value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work(accepted_value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        mock.received
            .work(accepted_value, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn work_first_type_str_Ok() {
        // Arrange
        let mock = TraitMock::<_, ()>::new();
        let accepted_value = "quo vadis";
        let returned_value = "veridis quo";
        mock.setup.work(accepted_value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work(accepted_value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        mock.received
            .work(accepted_value, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn work_first_type_explicit_type_Ok() {
        // Arrange
        let mock = TraitMock::<u8, Box<Vec<String>>>::new();
        let accepted_value = 10u8;
        let returned_value = 20u8;
        mock.setup.work(accepted_value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work(accepted_value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        mock.received
            .work(accepted_value, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn work_and_rest_work_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let accepted_work_value = 10;
        let returned_work_value = 20;
        let accepted_rest_value = 15;
        let returned_rest_value = "quo vadis";
        mock.setup
            .work(accepted_work_value)
            .returns(returned_work_value)
            .rest(accepted_rest_value)
            .returns(returned_rest_value);

        // Act
        let actual_returned_work_value = mock.work(accepted_work_value);
        let actual_returned_rest_value = mock.rest(accepted_rest_value);

        // Assert
        assert_eq!(returned_work_value, actual_returned_work_value);
        assert_eq!(returned_rest_value, actual_returned_rest_value);

        mock.received
            .work(accepted_work_value, Times::Once)
            .rest(accepted_rest_value, Times::Once)
            .no_other_calls();
    }
}
