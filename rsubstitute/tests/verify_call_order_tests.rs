use rsubstitute::*;

#[mock]
fn foo() {}

#[mock]
fn bar() {}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn NoVerification_Correct() {
        // Arrange
        // Act
        foo();
        bar();

        // Assert
        bar::received(Times::Once).no_other_calls();
        foo::received(Times::Once).no_other_calls();
    }

    #[test]
    fn WithVerification_Throws() {
        // Arrange
        // Act
        foo();
        bar();

        // Assert
        let panic_msg = not_enough_asserts::record_panic(|| {
            verify_call_order(|| {
                bar::received(Times::Once).no_other_calls();
                foo::received(Times::Once).no_other_calls();
            });
        });
        assert!(panic_msg.is_some());
    }
}
