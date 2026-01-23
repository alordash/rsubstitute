use rsubstitute::macros::mock;

#[mock]
fn get_return<T>(value: T) -> T {
    return value;
}

#[mock]
fn return_constraint<T: Default>() -> T {
    return T::default();
}

mod tests {
    #![allow(unused_variables)]
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::assertions::assert_panics;
    use rsubstitute::*;

    mod get_return_tests {
        use super::*;

        #[test]
        fn get_return_Ok() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);

            // Act
            let actual_returned_number = get_return(accepted_number);

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();
        }

        #[test]
        fn get_return_TwoTypes_Ok() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);

            let accepted_str = "quo vadis";
            let returned_str = "veridis quo";
            get_return::setup(accepted_str).returns(returned_str);

            // Act
            let actual_returned_number = get_return(accepted_number);
            let actual_returned_str = get_return(accepted_str);

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();

            assert_eq!(returned_str, actual_returned_str);
            get_return::received(accepted_str, Times::Once).no_other_calls();
        }

        #[test]
        fn get_return_UnconfiguredType_PanicsOk() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);

            // Act
            let actual_returned_number = get_return(accepted_number);
            assert_panics(
                || get_return("str wasn't configured"),
                "No fn configuration found for this call! TODO: write call description",
            );

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();
        }

        #[test]
        fn get_return_CallBase_Ok() {
            // Arrange
            let accepted_number = 33;
            get_return::setup(accepted_number).call_base();

            // Act
            let returned_number = get_return(accepted_number);

            // Assert
            assert_eq!(accepted_number, returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();
        }
    }

    mod return_constraint_tests {
        use super::*;

        #[test]
        fn return_constraint_Ok() {
            // Arrange
            let returned_number_value = 100;
            return_constraint::setup().returns(returned_number_value);
            let returned_string_value = String::from("quo vadis");
            return_constraint::setup().returns(returned_string_value.clone());

            // Act
            let actual_number_value = return_constraint();
            let actual_string_value: String = return_constraint();

            // Assert
            assert_eq!(returned_number_value, actual_number_value);
            return_constraint::received::<i32>(Times::Once).no_other_calls();

            assert_eq!(returned_string_value, actual_string_value);
            return_constraint::received::<String>(Times::Once).no_other_calls();

            return_constraint::received::<()>(Times::Never).no_other_calls();
        }
    }
}
