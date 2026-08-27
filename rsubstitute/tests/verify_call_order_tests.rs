use rsubstitute::*;

#[mock]
fn foo<T>(_: T) {}

#[mock]
fn bar<T>(_: T) {}

#[mock]
trait Trait {
    fn traiting(&self, _: i32);
}

#[mock]
struct Struct;

#[mock(base)]
impl Struct {
    fn new() -> Self {
        Self
    }

    fn structing(&self, _: i32) {}
}

#[mock(base)]
impl Trait for Struct {
    fn traiting(&self, _: i32) {}
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::assert_panics;

    #[test]
    fn OnlyFn_NoVerificationValidOrder_Ok() {
        // Arrange
        // Act
        foo(1);
        foo(2);
        foo("amogus");
        bar(false);

        // Assert
        foo::received(1, Times::Once).no_other_calls();
        foo::received(2, Times::Once);
        foo::received("amogus", Times::Once);
        bar::received(false, Times::Once).no_other_calls();
    }

    #[test]
    fn OnlyFn_NoVerificationInvalidOrder_Ok() {
        // Arrange
        // Act
        foo(1);
        foo(2);
        foo("amogus");
        bar(false);

        // Assert
        bar::received(false, Times::Once).no_other_calls();
        foo::received("amogus", Times::Once);
        foo::received(2, Times::Once);
        foo::received(1, Times::Once).no_other_calls();
    }

    #[test]
    fn OnlyFn_WithVerificationValidOrder_Ok() {
        // Arrange
        // Act
        foo(1);
        foo(2);
        foo("amogus");
        bar(false);

        // Assert
        verify_call_order(|| {
            foo::received(1, Times::Once).no_other_calls();
            foo::received(2, Times::Once);
            foo::received("amogus", Times::Once);
            bar::received(false, Times::Once).no_other_calls();
        });
    }

    // TODO - add test WithVerification that doesn't panic
    #[test]
    fn OnlyFn_WithVerification_Panics() {
        // Arrange
        // Act
        foo(1);
        foo(2);
        foo("amogus");
        bar(false);

        // Assert
        assert_panics(
            || {
                verify_call_order(|| {
                    bar::received(false, Times::Once).no_other_calls();
                    foo::received("amogus", Times::Once);
                    foo::received(2, Times::Once);
                    foo::received(1, Times::Once).no_other_calls();
                })
            },
            r#"Expected to receive these calls in order:

	bar(false)
	foo("amogus")
	foo(2)
	foo(1)

Actually received matching calls in this order:

	foo(1)
	foo(2)
	foo("amogus")
	bar(false)
"#,
        );
    }

    #[test]
    fn AllTogether_NoVerification_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let mut struct_mock = Struct::new();

        // Act
        foo(1);
        trait_mock.traiting(2);
        struct_mock.structing(3);
        bar(4);
        struct_mock.traiting(5);

        // Assert
        struct_mock.received().as_Trait().traiting(5, Times::Once);
        bar::received(4, Times::Once).no_other_calls();
        struct_mock
            .received()
            .structing(3, Times::Once)
            .no_other_calls();
        trait_mock
            .received()
            .traiting(2, Times::Once)
            .no_other_calls();
        foo::received(1, Times::Once).no_other_calls();
    }

    #[test]
    fn AllTogether_WithVerification_Panics() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let mut struct_mock = Struct::new();

        // Act
        foo(1);
        trait_mock.traiting(2);
        struct_mock.structing(3);
        bar(4);
        struct_mock.traiting(5);

        // Assert
        verify_call_order(|| {
            struct_mock.received().as_Trait().traiting(5, Times::Once);
            bar::received(4, Times::Once).no_other_calls();
            struct_mock
                .received()
                .structing(3, Times::Once)
                .no_other_calls();
            trait_mock
                .received()
                .traiting(2, Times::Once)
                .no_other_calls();
            foo::received(1, Times::Once).no_other_calls();
        });
    }
}
