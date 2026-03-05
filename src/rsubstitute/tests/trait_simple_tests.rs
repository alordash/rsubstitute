use rsubstitute::macros::mock;

#[mock]
trait Trait {
    fn f(&self);

    #[allow(unused)]
    fn mutate(&mut self);

    #[allow(unused)]
    fn consume(self) -> i32;
}

#[cfg(not(test))]
#[allow(unused)]
trait AnotherTestTrait {}
#[cfg(test)]
#[allow(unused)]
trait AnotherTestTrait {}
#[cfg(test)]
pub use __rsubstitute_generated_AnotherTestTrait::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_AnotherTestTrait {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct AnotherTestTraitMockData<'rs> {}
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct AnotherTestTraitMockSetup<'rs> {
        data: Arc<AnotherTestTraitMockData<'rs>>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct AnotherTestTraitMockReceived<'rs> {
        data: Arc<AnotherTestTraitMockData<'rs>>,
    }
    #[derive(Clone)]
    pub struct AnotherTestTraitMock<'rs> {
        pub setup: AnotherTestTraitMockSetup<'rs>,
        pub received: AnotherTestTraitMockReceived<'rs>,
        data: Arc<AnotherTestTraitMockData<'rs>>,
    }
    impl<'rs> AnotherTestTrait for AnotherTestTraitMock<'rs> {}
    impl<'rs> AnotherTestTraitMock<'rs> {
        pub fn new() -> Self {
            let data = Arc::new(AnotherTestTraitMockData {});
            return AnotherTestTraitMock {
                setup: AnotherTestTraitMockSetup { data: data.clone() },
                received: AnotherTestTraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'rs> AnotherTestTraitMockSetup<'rs> {}
    impl<'rs> AnotherTestTraitMockReceived<'rs> {
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}

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
            mock.setup.mutate().does(move || flag_copy.set(true));
            mock.setup.consume().returns(33);
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
        mock.setup.f().returns(return_value).and_does(move |_| {
            *callback_flag_clone.borrow_mut() = true
        });

        // Act
        let result = mock.f();

        // Assert
        assert_eq!((), result);
        assert!(*callback_flag.borrow());
        mock.received.f(Times::Once).no_other_calls();
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

        mock.received.f(Times::Exactly(3)).no_other_calls();
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
            || mock.received.f(Times::Once),
            r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received.f(Times::Exactly(1)),
            r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received.f(Times::Exactly(2)),
            r#"Expected to receive a call 2 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || mock.received.f(Times::Exactly(4)),
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
