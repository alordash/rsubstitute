use rsubstitute::prelude::mock;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[rustfmt::skip]
#[allow(unused)]
mod consts {
    pub const BY_VALUE_VALUE:                       i32 = 1 ;
    pub const BY_VALUE_COLON_VALUE:                 i32 = 2 ;
    pub const BY_MUT_VALUE_VALUE:                   i32 = 3 ;
    pub const BY_MUT_VALUE_COLON_VALUE:             i32 = 4 ;
    pub const BY_REF_VALUE:                         i32 = 5 ;
    pub const BY_REF_COLON_VALUE:                   i32 = 6 ;
    pub const BY_REF_WITH_LIFETIME_VALUE:           i32 = 7 ;
    pub const BY_REF_COLON_WITH_LIFETIME_VALUE:     i32 = 8 ;
    pub const BY_REF_MUT_VALUE:                     i32 = 9 ;
    pub const BY_REF_MUT_COLON_VALUE:               i32 = 10;
    pub const BY_REF_MUT_WITH_LIFETIME_VALUE:       i32 = 11;
    pub const BY_REF_MUT_COLON_WITH_LIFETIME_VALUE: i32 = 12;
    pub const BY_BOX_VALUE:                         i32 = 13;
    pub const BY_RC_VALUE:                          i32 = 14;
    pub const BY_ARC_VALUE:                         i32 = 15;
    pub const BY_PIN_VALUE:                         i32 = 16;
    pub const NESTED_VALUE:                         i32 = 17;
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

    fn by_box(self: Box<Self>) {}
    fn by_box_mut(mut self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_rc_mut(mut self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_arc_mut(mut self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn by_pin_mut(mut self: Pin<&Self>) {}
    fn by_pin_with_lifetime<'a>(self: Pin<&'a Self>) {}
    fn by_pin_mut_with_lifetime<'a>(mut self: Pin<&'a Self>) {}

    fn nested<'a>(self: &'a Box<Pin<Rc<Box<Arc<Pin<Rc<Self>>>>>>>) {}

    fn return_by_value(self) -> i32 {
        BY_VALUE_VALUE
    }
    fn return_by_value_colon(self: Self) -> i32 {
        BY_VALUE_COLON_VALUE
    }
    fn return_by_mut_value(#[allow(unused_mut)] mut self) -> i32 {
        BY_MUT_VALUE_VALUE
    }
    fn return_by_mut_value_colon(#[allow(unused_mut)] mut self: Self) -> i32 {
        BY_MUT_VALUE_COLON_VALUE
    }

    fn return_by_ref(&self) -> i32 {
        BY_REF_VALUE
    }
    fn return_by_ref_colon(self: &Self) -> i32 {
        BY_REF_COLON_VALUE
    }
    fn return_by_ref_with_lifetime<'a>(&'a self) -> i32 {
        BY_REF_WITH_LIFETIME_VALUE
    }
    fn return_by_ref_colon_with_lifetime<'a>(self: &'a Self) -> i32 {
        BY_REF_COLON_WITH_LIFETIME_VALUE
    }
    fn return_by_ref_mut(&mut self) -> i32 {
        BY_REF_MUT_VALUE
    }
    fn return_by_ref_mut_colon(self: &mut Self) -> i32 {
        BY_REF_MUT_COLON_VALUE
    }
    fn return_by_ref_mut_with_lifetime<'a>(&'a mut self) -> i32 {
        BY_REF_MUT_WITH_LIFETIME_VALUE
    }
    fn return_by_ref_mut_colon_with_lifetime<'a>(self: &'a mut Self) -> i32 {
        BY_REF_MUT_COLON_WITH_LIFETIME_VALUE
    }

    fn return_by_box(self: Box<Self>) -> i32 {
        BY_BOX_VALUE
    }
    fn return_by_rc(self: Rc<Self>) -> i32 {
        BY_RC_VALUE
    }
    fn return_by_arc(self: Arc<Self>) -> i32 {
        BY_ARC_VALUE
    }
    fn return_by_pin(self: Pin<&Self>) -> i32 {
        BY_PIN_VALUE
    }

    fn return_nested<'a>(self: &'a Box<Pin<Rc<Box<Arc<Pin<Rc<Self>>>>>>>) -> i32 {
        NESTED_VALUE
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    #![allow(unused_imports)]

    use super::*;
    use rsubstitute::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // TODO - extract to some helpers?
    // Helper struct for callbacks verification.
    // Leaking for ability to `Copy` so no need to create clones for moving them in closures.
    #[derive(Copy, Clone)]
    struct Counter(&'static AtomicUsize);
    impl Counter {
        fn new() -> Self {
            Self(Box::leak(Box::new(AtomicUsize::new(0))))
        }
        fn inc(&self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
        fn get(&self) -> usize {
            self.0.load(Ordering::Relaxed)
        }
    }

    #[test]
    fn by_value_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_value().does(move |_| counter.inc());
        mock.setup
            .by_value()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.clone().by_value();
        mock.clone().by_value();

        // Assert
        mock.received.by_value(Times::Exactly(2)).no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_value_colon_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_value_colon().does(move |_| counter.inc());
        mock.setup
            .by_value_colon()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        // TODO - test and write in doc about the ability to clone mocks and what does it mean
        mock.clone().by_value_colon();
        mock.clone().by_value_colon();

        // Assert
        mock.received
            .by_value_colon(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_mut_value_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_mut_value().does(move |_| counter.inc());
        mock.setup
            .by_mut_value()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.clone().by_mut_value();
        mock.clone().by_mut_value();

        // Assert
        mock.received
            .by_mut_value(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_mut_value_colon_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_mut_value_colon().does(move |_| counter.inc());
        mock.setup
            .by_mut_value_colon()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.clone().by_mut_value_colon();
        mock.clone().by_mut_value_colon();

        // Assert
        mock.received
            .by_mut_value_colon(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_ref_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_ref().does(move |_| counter.inc());
        mock.setup
            .by_ref()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.by_ref();
        mock.by_ref();

        // Assert
        mock.received.by_ref(Times::Exactly(2)).no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_ref_colon_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_ref_colon().does(move |_| counter.inc());
        mock.setup
            .by_ref_colon()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.by_ref_colon();
        mock.by_ref_colon();

        // Assert
        mock.received
            .by_ref_colon(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_ref_with_lifetime_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup
            .by_ref_with_lifetime()
            .does(move |_| counter.inc());
        mock.setup
            .by_ref_with_lifetime()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.by_ref_with_lifetime();
        mock.by_ref_with_lifetime();

        // Assert
        mock.received
            .by_ref_with_lifetime(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_ref_colon_with_lifetime_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup
            .by_ref_colon_with_lifetime()
            .does(move |_| counter.inc());
        mock.setup
            .by_ref_colon_with_lifetime()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.by_ref_colon_with_lifetime();
        mock.by_ref_colon_with_lifetime();

        // Assert
        mock.received
            .by_ref_colon_with_lifetime(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_ref_mut_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_ref_mut().does(move |_| counter.inc());
        mock.setup
            .by_ref_mut()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.by_ref_mut();
        mock.by_ref_mut();

        // Assert
        mock.received.by_ref_mut(Times::Exactly(2)).no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_ref_mut_colon_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_ref_mut_colon().does(move |_| counter.inc());
        mock.setup
            .by_ref_mut_colon()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.by_ref_mut_colon();
        mock.by_ref_mut_colon();

        // Assert
        mock.received
            .by_ref_mut_colon(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_ref_mut_with_lifetime_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup
            .by_ref_mut_with_lifetime()
            .does(move |_| counter.inc());
        mock.setup
            .by_ref_mut_with_lifetime()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.by_ref_mut_with_lifetime();
        mock.by_ref_mut_with_lifetime();

        // Assert
        mock.received
            .by_ref_mut_with_lifetime(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_ref_mut_colon_with_lifetime_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup
            .by_ref_mut_colon_with_lifetime()
            .does(move |_| counter.inc());
        mock.setup
            .by_ref_mut_colon_with_lifetime()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.by_ref_mut_colon_with_lifetime();
        mock.by_ref_mut_colon_with_lifetime();

        // Assert
        mock.received
            .by_ref_mut_colon_with_lifetime(Times::Exactly(2))
            .no_other_calls();
        assert_eq!(2, counter.get());
    }

    #[test]
    fn by_box_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let counter = Counter::new();
        mock.setup.by_value().does(move |_| counter.inc());
        mock.setup
            .by_value()
            .call_base()
            .and_does(move |_| counter.inc());

        // Act
        mock.clone().by_value();
        mock.clone().by_value();

        // Assert
        mock.received.by_value(Times::Exactly(2)).no_other_calls();
        assert_eq!(2, counter.get());
    }
}
