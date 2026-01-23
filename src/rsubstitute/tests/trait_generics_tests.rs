use rsubstitute::macros::mock;

#[mock]
trait Trait<T1, T2> {
    fn get_return(&self, value: T1) -> T1;

    // TODO - add support and write test for it
    // fn return_where_constraint(&self) -> T1
    // where
    //     T1: Default;

    fn get_return_different(&self, value: T1) -> T2;
}

mod trait_generic_tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    mod get_return_tests {
        use super::*;

        #[test]
        fn get_return_first_type_i32_Ok() {
            // Arrange
            let mock = TraitMock::<_, String>::new();
            let accepted_value = 10;
            let returned_value = 200;
            mock.setup
                .get_return(accepted_value)
                .returns(returned_value);

            // Act
            let actual_returned_value = mock.get_return(accepted_value);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            mock.received
                .get_return(accepted_value, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn get_return_first_type_str_Ok() {
            // Arrange
            let mock = TraitMock::<_, ()>::new();
            let accepted_value = "quo vadis";
            let returned_value = "veridis quo";
            mock.setup
                .get_return(accepted_value)
                .returns(returned_value);

            // Act
            let actual_returned_value = mock.get_return(accepted_value);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            mock.received
                .get_return(accepted_value, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn get_return_first_type_explicit_type_Ok() {
            // Arrange
            let mock = TraitMock::<u8, Box<Vec<String>>>::new();
            let accepted_value = 10u8;
            let returned_value = 20u8;
            mock.setup
                .get_return(accepted_value)
                .returns(returned_value);

            // Act
            let actual_returned_value = mock.get_return(accepted_value);

            // Assert
            assert_eq!(returned_value, actual_returned_value);

            mock.received
                .get_return(accepted_value, Times::Once)
                .no_other_calls();
        }
    }

    #[test]
    fn get_return_and_get_return_different_get_return_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let accepted_get_return_value = 10;
        let returned_get_return_value = 20;
        let accepted_get_return_different_value = 15;
        let returned_get_return_different_value = "quo vadis";
        mock.setup
            .get_return(accepted_get_return_value)
            .returns(returned_get_return_value)
            .get_return_different(accepted_get_return_different_value)
            .returns(returned_get_return_different_value);

        // Act
        let actual_returned_get_return_value = mock.get_return(accepted_get_return_value);
        let actual_returned_get_return_different_value =
            mock.get_return_different(accepted_get_return_different_value);

        // Assert
        assert_eq!(returned_get_return_value, actual_returned_get_return_value);
        assert_eq!(
            returned_get_return_different_value,
            actual_returned_get_return_different_value
        );

        mock.received
            .get_return(accepted_get_return_value, Times::Once)
            .get_return_different(accepted_get_return_different_value, Times::Once)
            .no_other_calls();
    }
}
