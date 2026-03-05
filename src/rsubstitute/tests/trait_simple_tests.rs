use rsubstitute::macros::mock;

#[mock]
trait Trait {
    fn f(&self);

    #[allow(unused)]
    fn mutate(&mut self);

    #[allow(unused)]
    fn consume(self) -> i32;
}

#[mock]
#[allow(unused)]
trait AnotherTestTrait {}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;
    use std::cell::{Cell, RefCell};
    use std::sync::Arc;

    #[test]
    fn no_support() {
        // Arrange
        let flag = Arc::new(Cell::new(false));
        let flag_copy = flag.clone();
        let mut mock = TraitMock::new();
        {
            mock.setup().mutate().does(move || flag_copy.set(true));
            mock.setup().consume().returns(33);
        }

        // mock.mutate();
        assert!(!flag.get());
        
        // let v = mock.consume();
        // assert_eq!(33, v);
    }

    #[test]
    fn f_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let callback_flag = Arc::new(RefCell::new(false));
        let callback_flag_clone = callback_flag.clone();
        let return_value = ();
        mock.setup().f().returns_and_does(return_value, move || {
            *callback_flag_clone.borrow_mut() = true
        });

        // Act
        let result = mock.f();

        // Assert
        assert_eq!((), result);
        assert!(*callback_flag.borrow());
        mock.received().f(Times::Once).no_other_calls();
    }

    #[test]
    fn f_NoConfig_Ok() {
        // Arrange
        let mock = TraitMock::new();

        // Act
        let result = mock.f();

        // Assert
        assert_eq!((), result);
    }

    #[test]
    fn f_MultipleTimes_Ok() {
        // Arrange
        let mock = TraitMock::new();

        // Act
        let result1 = mock.f();
        let result2 = mock.f();
        let result3 = mock.f();

        // Assert
        assert_eq!((), result1);
        assert_eq!((), result2);
        assert_eq!((), result3);

        mock.received().f(Times::Exactly(3)).no_other_calls();
    }

    #[test]
    fn f_MultipleTimes_Panics() {
        // Arrange
        let mock = TraitMock::new();

        // Act
        mock.f();
        mock.f();
        mock.f();

        // Assert
        assert_panics(
            || mock.received().f(Times::Once),
            r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received().f(Times::Exactly(1)),
            r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received().f(Times::Exactly(2)),
            r#"Expected to receive a call 2 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received().f(Times::Exactly(4)),
            r#"Expected to receive a call 4 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );
    }
}
