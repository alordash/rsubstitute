use rsubstitute::macros::*;

trait FirstTrait {
    fn get(&self) -> i32;
}

trait SecondTrait {
    fn get(&self) -> &str;
}

pub const DEFAULT_STRUCT_GET_VALUE: i32 = 200;
pub const DEFAULT_FIRST_TRAIT_GET_VALUE: i32 = 500;
pub const DEFAULT_SECOND_TRAIT_GET_VALUE: &'static str = "quo vadis";

// TODO - dont add "Mock" suffix to generated struct mock
mocked_base! {
    struct Struct;

    impl Struct {
        pub fn new() -> Self { Self }

        pub fn get(&self) -> i32 {
            DEFAULT_STRUCT_GET_VALUE
        }

        pub fn get_plus_one(&self) -> i32 {
            let value = self.get() + FirstTrait::get(self);
            return value;
        }
    }

    impl FirstTrait for Struct {
        fn get(&self) -> i32 {
            DEFAULT_FIRST_TRAIT_GET_VALUE
        }
    }

    impl SecondTrait for Struct {
        fn get(&self) -> &str {
            DEFAULT_SECOND_TRAIT_GET_VALUE
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    #[test]
    fn get_plus_one_Ok() {
        // Arrange
        let mock = StructMock::new();

        let value = 302;
        mock.setup.get_plus_one().returns(value);

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        assert_eq!(value, actual_value);

        mock.received.get_plus_one(Times::Once).no_other_calls();
    }

    #[test]
    fn get_plus_one_CallBase_Ok() {
        // Arrange
        let mock = StructMock::new();

        let struct_value = 302;
        let trait_value = 33;
        mock.setup
            .get()
            .returns(struct_value)
            .get_plus_one()
            .call_base()
            .and_does(|mock, _| {
                println!(
                    "doing something, mock name: {}",
                    core::any::type_name_of_val(mock)
                )
            })
            .as_FirstTrait
            .get()
            .returns(trait_value);

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        let expected_value = struct_value + trait_value;
        assert_eq!(expected_value, actual_value);

        mock.received
            .get(Times::Once)
            .get_plus_one(Times::Once)
            .as_FirstTrait
            .get(Times::Once);
        mock.received.no_other_calls();
    }

    #[test]
    fn get_plus_one_StructCallBase_Ok() {
        // Arrange
        let mock = StructMock::new();

        let trait_value = 33;
        mock.setup
            .get()
            .call_base()
            .get_plus_one()
            .call_base()
            .as_FirstTrait
            .get()
            .returns(trait_value);

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        let expected_value = DEFAULT_STRUCT_GET_VALUE + trait_value;
        assert_eq!(expected_value, actual_value);

        mock.received
            .get(Times::Once)
            .get_plus_one(Times::Once)
            .as_FirstTrait
            .get(Times::Once);
        mock.received.no_other_calls();
    }

    #[test]
    fn get_plus_one_StructAndTraitCallBase_Ok() {
        // Arrange
        let mock = StructMock::new();

        mock.setup
            .get()
            .call_base()
            .get_plus_one()
            .call_base()
            .as_FirstTrait
            .get()
            .call_base();

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        let expected_value = DEFAULT_STRUCT_GET_VALUE + DEFAULT_FIRST_TRAIT_GET_VALUE;
        assert_eq!(expected_value, actual_value);

        mock.received
            .get(Times::Once)
            .get_plus_one(Times::Once)
            .as_FirstTrait
            .get(Times::Once);
        mock.received.no_other_calls();
    }

    #[test]
    fn get_SelfAndBothTraits_Ok() {
        // Arrange
        let mock = StructMock::new();

        let self_value = 5;
        let first_trait_value = 15;
        let second_trait_value = "veridis quo";
        let setup = &mock.setup;
        setup.get().returns(self_value);
        setup.as_FirstTrait.get().returns(first_trait_value);
        setup.as_SecondTrait.get().returns(second_trait_value);

        // Act
        let actual_self_value = mock.get();
        let actual_first_trait_value = FirstTrait::get(&mock);
        let actual_second_trait_value = (&mock as &dyn SecondTrait).get();

        // Assert
        assert_eq!(self_value, actual_self_value);
        assert_eq!(first_trait_value, actual_first_trait_value);
        assert_eq!(second_trait_value, actual_second_trait_value);

        let received = &mock.received;
        received.get(Times::Once);
        received.as_FirstTrait.get(Times::Once);
        received.as_SecondTrait.get(Times::Once);
        received.no_other_calls();
    }
}
