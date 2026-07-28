mod test_utils;

use rsubstitute::*;

#[rustfmt::skip]
#[allow(unused)]
mod consts {
    pub const BY_VALUE:                                       i32 = 1 ;
    pub const BY_VALUE_COLON:                                 i32 = 2 ;
    pub const BY_MUT_VALUE:                                   i32 = 3 ;
    pub const BY_MUT_VALUE_COLON:                             i32 = 4 ;
    pub const BY_REF:                                         i32 = 5 ;
    pub const BY_REF_COLON:                                   i32 = 6 ;
    pub const BY_REF_WITH_LIFETIME:                           i32 = 7 ;
    pub const BY_REF_COLON_WITH_LIFETIME:                     i32 = 8 ;
    pub const BY_REF_MUT:                                     i32 = 9 ;
    pub const BY_REF_MUT_COLON:                               i32 = 10;
    pub const BY_REF_MUT_WITH_LIFETIME:                       i32 = 11;
    pub const BY_REF_MUT_COLON_WITH_LIFETIME:                 i32 = 12;
}
use consts::*;

#[rustfmt::skip]
#[mock(base)]
#[allow(unused_mut)] // TODO - this should disable warnings
trait Trait: Sized {
    fn by_value          (    self      ) {}
    fn by_value_colon    (    self: Self) {}
    fn by_mut_value      (mut self      ) {}
    fn by_mut_value_colon(mut self: Self) {}

    fn by_ref                            (&       self              ) {}
    fn by_ref_colon                      (        self: &       Self) {}
    fn by_ref_with_lifetime          <'a>(&'a     self              ) {}
    fn by_ref_colon_with_lifetime    <'a>(        self: &'a     Self) {}
    fn by_ref_mut                        (&   mut self              ) {}
    fn by_ref_mut_colon                  (        self: &   mut Self) {}
    fn by_ref_mut_with_lifetime      <'a>(&'a mut self              ) {}
    fn by_ref_mut_colon_with_lifetime<'a>(        self: &'a mut Self) {}

    fn return_by_value          (    self      ) -> i32 { BY_VALUE }
    fn return_by_value_colon    (    self: Self) -> i32 { BY_VALUE_COLON }
    fn return_by_mut_value      (mut self      ) -> i32 { BY_MUT_VALUE }
    fn return_by_mut_value_colon(mut self: Self) -> i32 { BY_MUT_VALUE_COLON }

    fn return_by_ref                            (&       self              ) -> i32 { BY_REF }
    fn return_by_ref_colon                      (        self: &       Self) -> i32 { BY_REF_COLON }
    fn return_by_ref_with_lifetime          <'a>(&'a     self              ) -> i32 { BY_REF_WITH_LIFETIME }
    fn return_by_ref_colon_with_lifetime    <'a>(        self: &'a     Self) -> i32 { BY_REF_COLON_WITH_LIFETIME }
    fn return_by_ref_mut                        (&   mut self              ) -> i32 { BY_REF_MUT }
    fn return_by_ref_mut_colon                  (        self: &   mut Self) -> i32 { BY_REF_MUT_COLON }
    fn return_by_ref_mut_with_lifetime      <'a>(&'a mut self              ) -> i32 { BY_REF_MUT_WITH_LIFETIME }
    fn return_by_ref_mut_colon_with_lifetime<'a>(        self: &'a mut Self) -> i32 { BY_REF_MUT_COLON_WITH_LIFETIME }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    #![allow(unused_imports)]

    use super::*;
    use rsubstitute::*;
    use test_utils::*;

    mod basic_tests {
        use super::*;

        #[test]
        fn by_value_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup().by_value().does(move |_, _| counter.inc());
            mock.setup()
                .by_value()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.clone().by_value();
            mock.clone().by_value();

            // Assert
            mock.received().by_value(Times::Exactly(2)).no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_value_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_value_colon()
                .does(move |_, _| counter.inc());
            mock.setup()
                .by_value_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            // TODO - test and write in doc about the ability to clone mocks and what does it mean
            mock.clone().by_value_colon();
            mock.clone().by_value_colon();

            // Assert
            mock.received()
                .by_value_colon(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_mut_value_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup().by_mut_value().does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_value()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.clone().by_mut_value();
            mock.clone().by_mut_value();

            // Assert
            mock.received()
                .by_mut_value(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_mut_value_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_mut_value_colon()
                .does(move |_, _| counter.inc());
            mock.setup()
                .by_mut_value_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.clone().by_mut_value_colon();
            mock.clone().by_mut_value_colon();

            // Assert
            mock.received()
                .by_mut_value_colon(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup().by_ref().does(move |_, _| counter.inc());
            mock.setup()
                .by_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref();
            mock.by_ref();

            // Assert
            mock.received().by_ref(Times::Exactly(2)).no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_ref_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup().by_ref_colon().does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_colon();
            mock.by_ref_colon();

            // Assert
            mock.received()
                .by_ref_colon(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_ref_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_with_lifetime()
                .does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_with_lifetime();
            mock.by_ref_with_lifetime();

            // Assert
            mock.received()
                .by_ref_with_lifetime(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_ref_colon_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_colon_with_lifetime()
                .does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_colon_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_colon_with_lifetime();
            mock.by_ref_colon_with_lifetime();

            // Assert
            mock.received()
                .by_ref_colon_with_lifetime(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup().by_ref_mut().does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_mut();
            mock.by_ref_mut();

            // Assert
            mock.received()
                .by_ref_mut(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_ref_mut_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_mut_colon()
                .does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_mut_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_mut_colon();
            mock.by_ref_mut_colon();

            // Assert
            mock.received()
                .by_ref_mut_colon(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_ref_mut_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_mut_with_lifetime()
                .does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_mut_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_mut_with_lifetime();
            mock.by_ref_mut_with_lifetime();

            // Assert
            mock.received()
                .by_ref_mut_with_lifetime(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }

        #[test]
        fn by_ref_mut_colon_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            mock.setup()
                .by_ref_mut_colon_with_lifetime()
                .does(move |_, _| counter.inc());
            mock.setup()
                .by_ref_mut_colon_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            mock.by_ref_mut_colon_with_lifetime();
            mock.by_ref_mut_colon_with_lifetime();

            // Assert
            mock.received()
                .by_ref_mut_colon_with_lifetime(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(2, counter.get());
        }
    }

    mod return_tests {
        use super::*;

        #[test]
        fn return_by_value_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_value()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_value()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.clone().return_by_value();
            let actual_second_value = mock.clone().return_by_value();

            // Assert
            mock.received()
                .return_by_value(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_VALUE, actual_second_value);
        }

        #[test]
        fn return_by_value_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_value_colon()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_value_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.clone().return_by_value_colon();
            let actual_second_value = mock.clone().return_by_value_colon();

            // Assert
            mock.received()
                .return_by_value_colon(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_VALUE_COLON, actual_second_value);
        }

        #[test]
        fn return_by_mut_value_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_value()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_value()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.clone().return_by_mut_value();
            let actual_second_value = mock.clone().return_by_mut_value();

            // Assert
            mock.received()
                .return_by_mut_value(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_VALUE, actual_second_value);
        }

        #[test]
        fn return_by_mut_value_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_mut_value_colon()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_mut_value_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.clone().return_by_mut_value_colon();
            let actual_second_value = mock.clone().return_by_mut_value_colon();

            // Assert
            mock.received()
                .return_by_mut_value_colon(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_MUT_VALUE_COLON, actual_second_value);
        }

        #[test]
        fn return_by_ref_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.clone().return_by_ref();
            let actual_second_value = mock.clone().return_by_ref();

            // Assert
            mock.received()
                .return_by_ref(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF, actual_second_value);
        }

        #[test]
        fn return_by_ref_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_colon()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.clone().return_by_ref_colon();
            let actual_second_value = mock.clone().return_by_ref_colon();

            // Assert
            mock.received()
                .return_by_ref_colon(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_COLON, actual_second_value);
        }

        #[test]
        fn return_by_ref_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_with_lifetime()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.clone().return_by_ref_with_lifetime();
            let actual_second_value = mock.clone().return_by_ref_with_lifetime();

            // Assert
            mock.received()
                .return_by_ref_with_lifetime(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_WITH_LIFETIME, actual_second_value);
        }

        #[test]
        fn return_by_ref_colon_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_colon_with_lifetime()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_colon_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.clone().return_by_ref_colon_with_lifetime();
            let actual_second_value = mock.clone().return_by_ref_colon_with_lifetime();

            // Assert
            mock.received()
                .return_by_ref_colon_with_lifetime(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_COLON_WITH_LIFETIME, actual_second_value);
        }

        #[test]
        fn return_by_ref_mut_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_mut()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_mut()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_mut();
            let actual_second_value = mock.return_by_ref_mut();

            // Assert
            mock.received()
                .return_by_ref_mut(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_MUT, actual_second_value);
        }

        #[test]
        fn return_by_ref_mut_colon_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_mut_colon()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_mut_colon()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_mut_colon();
            let actual_second_value = mock.return_by_ref_mut_colon();

            // Assert
            mock.received()
                .return_by_ref_mut_colon(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_MUT_COLON, actual_second_value);
        }

        #[test]
        fn return_by_ref_mut_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_mut_with_lifetime()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_mut_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_mut_with_lifetime();
            let actual_second_value = mock.return_by_ref_mut_with_lifetime();

            // Assert
            mock.received()
                .return_by_ref_mut_with_lifetime(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_MUT_WITH_LIFETIME, actual_second_value);
        }

        #[test]
        fn return_by_ref_mut_colon_with_lifetime_Ok() {
            // Arrange
            let mut mock = TraitMock::new();
            let counter = Counter::new();
            let first_value = -100;
            mock.setup()
                .return_by_ref_mut_colon_with_lifetime()
                .returns(first_value)
                .and_does(move |_, _| counter.inc());
            mock.setup()
                .return_by_ref_mut_colon_with_lifetime()
                .call_base()
                .and_does(move |_, _| counter.double_inc());

            // Act
            let actual_first_value = mock.return_by_ref_mut_colon_with_lifetime();
            let actual_second_value = mock.return_by_ref_mut_colon_with_lifetime();

            // Assert
            mock.received()
                .return_by_ref_mut_colon_with_lifetime(Times::Exactly(2))
                .no_other_calls();
            assert_eq!(3, counter.get());
            assert_eq!(first_value, actual_first_value);
            assert_eq!(BY_REF_MUT_COLON_WITH_LIFETIME, actual_second_value);
        }
    }
}
