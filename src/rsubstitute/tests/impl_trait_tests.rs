use rsubstitute::*;

const DUMMY_VALUE: i32 = 124;
#[mock(base)]
trait Dummy {
    fn work(&self) -> i32 {
        DUMMY_VALUE
    }
}

#[mock]
fn f(dummy: impl Dummy) -> i32 {
    dummy.work()
}

#[mock]
trait Trait {
    fn work(&self, dummy: impl Dummy) -> i32 {
        dummy.work()
    }

    fn static_work(dummy: impl Dummy) -> i32 {
        dummy.work()
    }
}

#[mock]
struct Struct;
#[mock(base)]
impl Struct {
    pub fn new() -> Self {
        Self
    }
}

#[mock]
impl Struct {
    pub fn work_self(&self, dummy: impl Dummy) -> i32 {
        dummy.work()
    }

    pub fn static_work_self(dummy: impl Dummy) -> i32 {
        dummy.work()
    }
}

#[mock]
impl Trait for Struct {
    fn work(&self, dummy: impl Dummy) -> i32 {
        dummy.work()
    }

    fn static_work(dummy: impl Dummy) -> i32 {
        dummy.work()
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    const ACTUAL_VALUE: i32 = DUMMY_VALUE + 123;
    const ACTUAL_STATIC_VALUE: i32 = ACTUAL_VALUE + 555;

    mod fn_tests {
        use super::*;
        #[test]
        fn f_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            f::setup(Arg::Any).returns(ACTUAL_VALUE);

            // Act
            let result = f(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            f::received(Arg::Any, Times::Any).no_other_calls();
        }

        #[test]
        fn f_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().returns_always(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().returns_always(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            f::setup(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .setup(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = f(dummy1);
            let result2 = f(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            f::received(
                Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                Times::Once,
            )
            .received(
                Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                Times::Once,
            )
            .no_other_calls();
        }
    }

    mod trait_tests {
        use super::*;

        #[test]
        fn trait_work_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            let mut mock = TraitMock::new();
            mock.setup().work(Arg::Any).returns(ACTUAL_VALUE);

            // Act
            let result = mock.work(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            mock.received().work(Arg::Any, Times::Any).no_other_calls();
        }

        #[test]
        fn trait_work_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().returns_always(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().returns_always(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            let mut mock = TraitMock::new();
            mock.setup()
                .work(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .work(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = mock.work(dummy1);
            let result2 = mock.work(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            mock.received()
                .work(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .work(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn trait_static_work_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            TraitMock::static_setup()
                .static_work(Arg::Any)
                .returns(ACTUAL_VALUE);

            // Act
            let result = TraitMock::static_work(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            TraitMock::static_received()
                .static_work(Arg::Any, Times::Any)
                .no_other_calls();
        }

        #[test]
        fn trait_static_work_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().returns_always(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().returns_always(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            TraitMock::static_setup()
                .static_work(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .static_work(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = TraitMock::static_work(dummy1);
            let result2 = TraitMock::static_work(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            TraitMock::static_received()
                .static_work(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .static_work(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }
    }

    mod struct_tests {
        use super::*;

        #[test]
        fn struct_work_self_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            let mut mock = Struct::new();
            mock.setup().work_self(Arg::Any).returns(ACTUAL_VALUE);

            // Act
            let result = mock.work_self(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            mock.received()
                .work_self(Arg::Any, Times::Any)
                .no_other_calls();
        }

        #[test]
        fn struct_work_self_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().returns_always(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().returns_always(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            let mut mock = Struct::new();
            mock.setup()
                .work_self(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .work_self(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = mock.work_self(dummy1);
            let result2 = mock.work_self(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            mock.received()
                .work_self(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .work_self(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn struct_static_work_self_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            Struct::static_setup()
                .static_work_self(Arg::Any)
                .returns(ACTUAL_VALUE);

            // Act
            let result = Struct::static_work_self(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            Struct::static_received()
                .static_work_self(Arg::Any, Times::Any)
                .no_other_calls();
        }

        #[test]
        fn struct_static_work_self_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().returns_always(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().returns_always(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            Struct::static_setup()
                .static_work_self(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .static_work_self(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = Struct::static_work_self(dummy1);
            let result2 = Struct::static_work_self(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            Struct::static_received()
                .static_work_self(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .static_work_self(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn struct_as_trait_work_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            let mut mock = Struct::new();
            mock.setup().as_Trait().work(Arg::Any).returns(ACTUAL_VALUE);

            // Act
            let result = mock.work(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            mock.received()
                .as_Trait()
                .work(Arg::Any, Times::Any)
                .no_other_calls();
        }

        #[test]
        fn struct_as_trait_work_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().returns_always(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().returns_always(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            let mut mock = Struct::new();
            mock.setup()
                .as_Trait()
                .work(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .work(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = mock.work(dummy1);
            let result2 = mock.work(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            mock.received()
                .as_Trait()
                .work(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .work(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }

        #[test]
        fn struct_static_as_trait_work_Ok() {
            // Arrange
            let mut dummy = DummyMock::new();
            dummy.setup().work().call_base();

            Struct::static_setup()
                .as_Trait()
                .static_work(Arg::Any)
                .returns(ACTUAL_VALUE);

            // Act
            let result = Struct::static_work(dummy);

            // Assert
            assert_eq!(ACTUAL_VALUE, result);
            Struct::static_received()
                .as_Trait()
                .static_work(Arg::Any, Times::Any)
                .no_other_calls();
        }

        #[test]
        fn struct_static_as_trait_work_ArgPredicate_Ok() {
            // Arrange
            let dummy1_value = 1;
            let mut dummy1 = DummyMock::new();
            dummy1.setup().work().returns_always(dummy1_value);

            let dummy2_value = 2;
            let mut dummy2 = DummyMock::new();
            dummy2.setup().work().returns_always(dummy2_value);

            let mocked_dummy1_value = 10;
            let mocked_dummy2_value = 20;
            Struct::static_setup()
                .as_Trait()
                .static_work(Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value))
                .returns(mocked_dummy1_value)
                .static_work(Arg::Any)
                .returns(mocked_dummy2_value);

            // Act
            let result1 = Struct::static_work(dummy1);
            let result2 = Struct::static_work(dummy2);

            // Assert
            assert_eq!(mocked_dummy1_value, result1);
            assert_eq!(mocked_dummy2_value, result2);

            Struct::static_received()
                .as_Trait()
                .static_work(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy1_value),
                    Times::Once,
                )
                .static_work(
                    Arg::is(|p: &Box<dyn Dummy>| p.work() == dummy2_value),
                    Times::Once,
                )
                .no_other_calls();
        }
    }
}
