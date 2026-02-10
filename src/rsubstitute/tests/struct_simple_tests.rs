use rsubstitute::macros::*;

mocked! {
    struct Struct {
        pub value: i32
    }

    impl Struct {
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        pub fn f(&self) {}
    }

    #[unmock]
    impl Struct {
        pub fn non_associative() {}
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;
    use std::cell::RefCell;
    use std::sync::Arc;

    #[test]
    fn f_Ok() {
        // Arrange
        let value = 22;
        let mock = StructMock::new(value);
        
        // Act
        StructMock::non_associative();
        
    }
}
