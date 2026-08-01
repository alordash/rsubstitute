use rsubstitute::*;
use std::marker::PhantomData;

trait Trait {
    fn f(&self);
}

// TODO - write tests for all targets (fn, trait and struct) with default generic values
#[mock]
struct Struct<Q = i32> {
    pub phantom: core::marker::PhantomData<Q>,
    pub value: i32,
}

#[mock(base)]
impl<Q> Struct<Q> {
    pub fn new(value: i32) -> Self {
        Self {
            phantom: PhantomData,
            value,
        }
    }
}

#[mock]
impl Struct {
    pub fn f(&self) {}
}

#[mock]
impl Trait for Struct {
    fn f(&self) {}
}

impl Struct {
    pub fn non_associative() {}
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::*;
    use rsubstitute::*;
    use std::cell::RefCell;
    use std::sync::Arc;

    mod struct_tests {
        use super::*;

        #[test]
        fn f_Ok() {
            // Arrange
            let value = 22;
            let mut mock = Struct::new(value);
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            mock.setup()
                .f()
                .does(move |_, _| *callback_flag_clone.borrow_mut() = true);

            // Act
            Struct::non_associative();
            let result = mock.f();

            // Assert
            assert_eq!(value, mock.value);
            assert_eq!((), result);
            assert!(*callback_flag.borrow());
            mock.received().f(Times::Once).no_other_calls();
        }

        #[test]
        fn f_NoConfig_Ok() {
            // Arrange
            let mut mock = Struct::new(1);

            // Act
            let result = mock.f();

            // Assert
            assert_eq!((), result);
            mock.received().f(Times::Once).no_other_calls();
        }

        #[test]
        fn f_MultipleTimes_Ok() {
            // Arrange
            let mut mock = Struct::new(1);

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
            let mut mock = Struct::new(1);

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

    mod trait_tests {
        use super::*;

        #[test]
        fn Trait_f_Ok() {
            // Arrange
            let value = 22;
            let mut mock = Struct::new(value);
            let callback_flag = Arc::new(RefCell::new(false));
            let callback_flag_clone = callback_flag.clone();
            mock.setup()
                .as_Trait()
                .f()
                .does(move |_, _| *callback_flag_clone.borrow_mut() = true);

            // Act
            Struct::non_associative();
            let result = Trait::f(&mock);

            // Assert
            assert_eq!(value, mock.value);
            assert_eq!((), result);
            assert!(*callback_flag.borrow());
            mock.received().as_Trait().f(Times::Once);
            mock.received().no_other_calls()
        }

        #[test]
        fn Trait_f_NoConfig_Ok() {
            // Arrange
            let mut mock = Struct::new(1);

            // Act
            let result = Trait::f(&mock);

            // Assert
            assert_eq!((), result);
            mock.received().as_Trait().f(Times::Once);
            mock.received().no_other_calls();
        }

        #[test]
        fn Trait_f_MultipleTimes_Ok() {
            // Arrange
            let mut mock = Struct::new(1);

            // Act
            let result1 = Trait::f(&mock);
            let result2 = Trait::f(&mock);
            let result3 = Trait::f(&mock);

            // Assert
            assert_eq!((), result1);
            assert_eq!((), result2);
            assert_eq!((), result3);

            mock.received().as_Trait().f(Times::Exactly(3));
            mock.received().no_other_calls();
        }

        #[test]
        fn Trait_f_MultipleTimes_Panics() {
            // Arrange
            let mut mock = Struct::new(1);

            // Act
            Trait::f(&mock);
            Trait::f(&mock);
            Trait::f(&mock);

            // Assert
            assert_panics(
                || mock.received().as_Trait().f(Times::Once),
                r#"Expected to receive a call exactly once matching:
	Trait::f()
Actually received 3 matching calls:
	Trait::f()
	Trait::f()
	Trait::f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().as_Trait().f(Times::Exactly(1)),
                r#"Expected to receive a call exactly once matching:
	Trait::f()
Actually received 3 matching calls:
	Trait::f()
	Trait::f()
	Trait::f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().as_Trait().f(Times::Exactly(2)),
                r#"Expected to receive a call 2 times matching:
	Trait::f()
Actually received 3 matching calls:
	Trait::f()
	Trait::f()
	Trait::f()
Received no non-matching calls"#,
            );

            assert_panics(
                || mock.received().as_Trait().f(Times::Exactly(4)),
                r#"Expected to receive a call 4 times matching:
	Trait::f()
Actually received 3 matching calls:
	Trait::f()
	Trait::f()
	Trait::f()
Received no non-matching calls"#,
            );
        }
    }
}
