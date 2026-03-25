use rsubstitute::prelude::*;

#[mock]
trait Trait<'a> {
    fn accept_ref<'b>(&self, r: &'a &&'b i32) -> i32;

    fn accept_ref_ptr<'b>(&self, r: &'a &*const &&'b i32) -> i32;

    fn generic<T1, T2>(&self, t1: T1) -> T2;
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::record_panic;

    #[test]
    fn accept_ref_NoConfig_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&&5;

        // Act
        let panic_msg = record_panic(|| mock.accept_ref(r));

        // Assert
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait::accept_ref({r})"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn accept_ref_DidNotReceive_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&&5;
        let r_ptr = core::ptr::from_ref(r);
        let return_value = 175;
        let unexpected_r = &&&14;
        let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

        mock.setup.accept_ref(r).returns(return_value);

        // Act
        let actual_return_value = mock.accept_ref(r);
        let panic_msg = record_panic(|| mock.received.accept_ref(unexpected_r, Times::Once));

        // Assert
        assert_eq!(return_value, actual_return_value);

        let expected_panic_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::accept_ref((&&&i32): equal to {unexpected_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
Trait::accept_ref(*{r}*)
	1. r (&&&i32):
		Expected reference (ptr: {unexpected_r_ptr:?}): {unexpected_r}
		Actual reference   (ptr: {r_ptr:?}): {r}"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn accept_ref_UnexpectedCall_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&&5;
        let return_value = 175;

        mock.setup.accept_ref(r).returns(return_value);

        // Act
        let actual_return_value = mock.accept_ref(r);
        let panic_msg = record_panic(|| mock.received.no_other_calls());

        // Assert
        assert_eq!(return_value, actual_return_value);

        let expected_panic_msg = format!(
            "Did not expect to receive any other calls. Received 1 unexpected call:
1. Trait::accept_ref({r})"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn accept_ref_ptr_NoConfig_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&(&&&5 as *const &&i32);

        // Act
        let panic_msg = record_panic(|| mock.accept_ref_ptr(r));

        // Assert
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait::accept_ref_ptr({r:?})"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn accept_ref_ptr_DidNotReceive_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&(&&&5 as *const &&i32);
        let r_ptr = core::ptr::from_ref(r);
        let return_value = 175;
        let unexpected_r = &&(&&&14 as *const &&i32);
        let unexpected_r_ptr = core::ptr::from_ref(unexpected_r);

        mock.setup.accept_ref_ptr(r).returns(return_value);

        // Act
        let actual_return_value = mock.accept_ref_ptr(r);
        let panic_msg = record_panic(|| mock.received.accept_ref_ptr(unexpected_r, Times::Once));

        // Assert
        assert_eq!(return_value, actual_return_value);

        let expected_panic_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::accept_ref_ptr((&&*const &&i32): equal to {unexpected_r:?})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
Trait::accept_ref_ptr(*{r:?}*)
	1. r (&&*const &&i32):
		Expected reference (ptr: {unexpected_r_ptr:?}): {unexpected_r:?}
		Actual reference   (ptr: {r_ptr:?}): {r:?}"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn accept_ref_ptr_UnexpectedCall_Ok() {
        // Arrange
        let mock = TraitMock::new();

        let r = &&(&&&5 as *const &&i32);
        let return_value = 175;

        mock.setup.accept_ref_ptr(r).returns(return_value);

        // Act
        let actual_return_value = mock.accept_ref_ptr(r);
        let panic_msg = record_panic(|| mock.received.no_other_calls());

        // Assert
        assert_eq!(return_value, actual_return_value);

        let expected_panic_msg = format!(
            "Did not expect to receive any other calls. Received 1 unexpected call:
1. Trait::accept_ref_ptr({r:?})"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn generic_NoConfig_Ok() {
        // Arrange
        let mock = TraitMock::new();
        type T1 = i32;
        type T2 = f64;
        let t1_name = core::any::type_name::<T1>();
        let t2_name = core::any::type_name::<T2>();
        let t1: T1 = 5;

        // Act
        let panic_msg = record_panic(|| mock.generic::<T1, T2>(t1));

        // Assert
        let expected_panic_msg = format!(
            "Mock wasn't configured to handle following call:
	Trait::generic<{t1_name}, {t2_name}>({t1})",
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }
    
    #[test]
    fn generic_DidNotReceiveSameGenerics_Ok() {
        // Arrange
        let mock = TraitMock::new();
        type T1 = i32;
        type T2 = f64;
        let t1_name = core::any::type_name::<T1>();
        let t2_name = core::any::type_name::<T2>();
        let t1: T1 = 5;
        let return_value: T2 = 64.0f64;
        let unexpected_t1: T1 = 235;
        
        mock.setup.generic(t1).returns(return_value);
        
        // Act
        let actual_return_value: T2 = mock.generic(t1);
        let panic_msg = record_panic(|| mock.received.generic::<T1, T2>(unexpected_t1, Times::Once));
        
        // Assert
        assert_eq!(return_value, actual_return_value);
        let expected_panic_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::generic<{t1_name}, {t2_name}>(({t1_name}): equal to {unexpected_t1})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
Trait::generic(*{t1}*)
	1. t1 ({t1_name}):
		Expected: {unexpected_t1}
		Actual:   {t1}"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }
    
    #[test]
    fn generic_DidNotReceiveDifferentGenerics_Ok() {
        // Arrange
        let mock = TraitMock::new();
        type T1 = i32;
        type T2 = f64; 
        type T3 = usize;
        type T4 = String;
        let t3_name = core::any::type_name::<T3>();
        let t4_name = core::any::type_name::<T4>();
        let t1: T1 = 5;
        let return_value: T2 = 64.0f64;
        let unexpected_t3: T3 = 11;
        
        mock.setup.generic(t1).returns(return_value);
        
        // Act
        let actual_return_value: T2 = mock.generic(t1);
        let panic_msg = record_panic(|| mock.received.generic::<T3, T4>(unexpected_t3, Times::Once));
        
        // Assert
        assert_eq!(return_value, actual_return_value);
        let expected_panic_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::generic<{t3_name}, {t4_name}>(({t3_name}): equal to {unexpected_t3})
Actually received no matching calls
Received no non-matching calls"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn generic_ref_UnexpectedCall_Ok() {
        // Arrange
        let mock = TraitMock::new();
        type T1 = i32;
        type T2 = f64;
        let t1_name = core::any::type_name::<T1>();
        let t2_name = core::any::type_name::<T2>();
        let t1: T1 = 5;
        let return_value: T2 = 64.0f64;

        mock.setup.generic(t1).returns(return_value);

        // Act
        let actual_return_value = mock.generic(t1);
        let panic_msg = record_panic(|| mock.received.no_other_calls());

        // Assert
        assert_eq!(return_value, actual_return_value);

        let expected_panic_msg = format!(
            "Did not expect to receive any other calls. Received 1 unexpected call:
1. Trait::generic<{t1_name}, {t2_name}>({t1})"
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }
}
