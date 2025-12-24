use rsubstitute_proc_macro::mock;

#[mock]
trait TestTrait {
    fn f(&self);
}

#[cfg(test)]
mod tests {
    use super::__rsubstitute_generated::*;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn f_test() {
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
}
