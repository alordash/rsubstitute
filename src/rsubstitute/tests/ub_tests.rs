use rsubstitute_proc_macro::mock;

#[mock]
trait Trait {
    fn work(&self, r: &i32);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    /// Invalid usage example.
    #[test]
    pub fn work_ReferenceToTemporaryValue_ReceivesDanglingReference() {
        // Arrange
        let mock = TraitMock::new();

        // Act
        static mut PTR_TO_TEMPORARY_VALUE: *const i32 = core::ptr::null();
        #[inline(never)]
        fn inner_call(mock: &TraitMock) {
            let temporary_value = 5;
            unsafe { PTR_TO_TEMPORARY_VALUE = &temporary_value };
            mock.work(&temporary_value);
        }
        inner_call(&mock);

        // Assert
        mock.received
            .work(
                Arg::is(|reference: &&i32| {
                    let actual_ptr_to_temporary_value = core::ptr::from_ref(*reference);
                    return unsafe { PTR_TO_TEMPORARY_VALUE == actual_ptr_to_temporary_value }
                        && **reference == -1;
                }),
                Times::Once,
            )
            .no_other_calls();
    }

    /// Fix by moving `received` section to the place where all arguments data is still alive.
    #[test]
    pub fn work_ReferenceToTemporaryValue_FixByReceivingBeforeDrop_ReceivesValidReference() {
        // Arrange
        let mock = TraitMock::new();

        // Act
        #[inline(never)]
        fn inner_call(mock: &TraitMock) {
            let temporary_value = 5;
            mock.work(&temporary_value);
            mock.received
                .work(
                    Arg::is(|reference: &&i32| {
                        return **reference == temporary_value;
                    }),
                    Times::Once,
                )
                .no_other_calls();
        }
        inner_call(&mock);

        // Assert
    }

    /// Fix by moving temporary value to the scope of mock.
    #[test]
    pub fn work_FixByNotUsingTemporaryValue_ReceivesValidReference() {
        // Arrange
        let mock = TraitMock::new();

        // Act
        let temporary_value = 5;
        mock.setup
            .work(&temporary_value)
            .does(|_| println!("amogus"));
        mock.work(&temporary_value);

        // Assert
        mock.received
            .work(&temporary_value, Times::Once)
            .no_other_calls();
    }
}
