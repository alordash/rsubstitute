use rsubstitute::macros::*;

trait Trait {
    fn get(&self) -> i32;
}

pub const DEFAULT_STRUCT_GET_VALUE: i32 = 200;
pub const DEFAULT_TRAIT_GET_VALUE: i32 = 500;

mocked! {
    struct Struct;

    impl Struct {
        pub fn new() -> Self { Self }

        fn get(&self) -> i32 {
            DEFAULT_STRUCT_GET_VALUE
        }

        pub fn get_plus_one(&self) -> i32 {
            let value = self.get() + Trait::get(self);
            return value;
        }
    }

    impl Trait for Struct {
        fn get(&self) -> i32 {
            DEFAULT_TRAIT_GET_VALUE
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
            .Trait
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
            .Trait
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
            .Trait
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
            .Trait
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
            .Trait
            .get()
            .call_base();

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        let expected_value = DEFAULT_STRUCT_GET_VALUE + DEFAULT_TRAIT_GET_VALUE;
        assert_eq!(expected_value, actual_value);

        mock.received
            .get(Times::Once)
            .get_plus_one(Times::Once)
            .Trait
            .get(Times::Once);
        mock.received.no_other_calls();
    }
}
