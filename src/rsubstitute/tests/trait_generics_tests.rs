use rsubstitute_proc_macro::mock;

#[mock]
trait Trait<T> {
    fn work(&self, value: T) -> T;

    fn flex(&self) -> T;
}

mod trait_generic_tests {
    use super::*;
    use rsubstitute_core::Times;

    #[test]
    fn work_i32_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let accepted_value = 10;
        let returned_value = 200;
        mock.setup.work(accepted_value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work(accepted_value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        mock.received.work(accepted_value, Times::Once);
    }

    #[test]
    fn work_str_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let accepted_value = "quo vadis";
        let returned_value = "veridis quo";
        mock.setup.work(accepted_value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work(accepted_value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        mock.received.work(accepted_value, Times::Once);
    }

    #[test]
    fn work_explicit_type_Ok() {
        // Arrange
        let mock = TraitMock::<u8>::new();
        let accepted_value = 10u8;
        let returned_value = 20u8;
        mock.setup.work(accepted_value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work(accepted_value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        mock.received.work(accepted_value, Times::Once);
    }
}
