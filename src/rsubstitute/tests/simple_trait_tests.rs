use rsubstitute_proc_macro::mock;

#[mock]
trait TestTrait {
    fn f(&self);
}

#[mock]
trait AnotherTestTrait {}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::__rsubstitute_generated_TestTrait::*;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn f_Correct() {
        // Arrange
        let mock = TestTraitMock::new();

        let callback_flag = Rc::new(RefCell::new(false));
        let callback_flag_clone = callback_flag.clone();
        let return_value = ();
        mock.f().returns_and_does(return_value, move || {
            *callback_flag_clone.borrow_mut() = true
        });

        // Act
        let result = TestTrait::f(&mock);

        // Assert
        assert_eq!((), result);
        assert!(*callback_flag.borrow());
    }

    #[test]
    fn f_NoConfig_Ok() {
        // Arrange
        let mock = TestTraitMock::new();

        // Act
        let result = TestTrait::f(&mock);

        // Assert
        assert_eq!((), result);
    }
}
