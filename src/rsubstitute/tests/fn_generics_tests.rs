use rsubstitute::macros::mock;
use std::fmt::{Debug, Display};

// #[mock]
// fn get_return<T>(value: T) -> T {
//     return value;
// }

#[mock]
fn return_constraint<T: Default>() -> T {
    return T::default();
}

#[mock]
fn return_where_constraint<T: Default>() -> T
where
    T: Debug,
{
    return T::default();
}

#[mock]
fn get_return_different<T1: Debug, T2>(_value: T1) -> T2
where
    T1: Clone + Display,
    T2: Default,
{
    T2::default()
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    mod get_return_tests {
        use super::*;

        #[test]
        fn get_return_Ok() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);

            // Act
            let actual_returned_number = get_return(accepted_number);

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();
        }

        #[test]
        fn get_return_TwoTypes_Ok() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);

            let accepted_str = "quo vadis";
            let returned_str = "veridis quo";
            get_return::setup(accepted_str).returns(returned_str);

            // Act
            let actual_returned_number = get_return(accepted_number);
            let actual_returned_str = get_return(accepted_str);

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();

            assert_eq!(returned_str, actual_returned_str);
            get_return::received(accepted_str, Times::Once).no_other_calls();
        }

        #[test]
        fn get_return_UnconfiguredType_Panics() {
            // Arrange
            let accepted_number = 10;
            let returned_number = 20;
            get_return::setup(accepted_number).returns(returned_number);
            let accepted_str = "str wasn't configured";

            // Act
            let actual_returned_number = get_return(accepted_number);
            let actual_error_msg = record_panic(|| get_return(accepted_str));

            // Assert
            assert_eq!(returned_number, actual_returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();

            let expected_error_msg = format!(
                "Mock wasn't configured to handle following call:
	get_return({accepted_str:?})"
            );
            assert_eq!(expected_error_msg, actual_error_msg);
        }

        #[test]
        fn get_return_CallBase_Ok() {
            // Arrange
            let accepted_number = 33;
            get_return::setup(accepted_number).call_base();

            // Act
            let returned_number = get_return(accepted_number);

            // Assert
            assert_eq!(accepted_number, returned_number);
            get_return::received(accepted_number, Times::Once).no_other_calls();
        }
    }

    mod return_constraint_tests {
        use super::*;

        #[test]
        fn return_constraint_Ok() {
            // Arrange
            let returned_number_value = 100;
            return_constraint::setup.returns(returned_number_value);
            let returned_string_value = String::from("quo vadis");
            return_constraint::setup.returns(returned_string_value.clone());

            // Act
            let actual_number_value = return_constraint();
            let actual_string_value: String = return_constraint();

            // Assert
            assert_eq!(returned_number_value, actual_number_value);
            return_constraint::received::<i32>(Times::Once).no_other_calls();

            assert_eq!(returned_string_value, actual_string_value);
            return_constraint::received::<String>(Times::Once).no_other_calls();

            return_constraint::received::<()>(Times::Never).no_other_calls();
        }

        #[test]
        fn return_constraint_CallBase_Ok() {
            // Arrange
            return_constraint::setup::<i32>().call_base();

            // Act
            let actual_number_value = return_constraint();

            // Assert
            assert_eq!(i32::default(), actual_number_value);
            return_constraint::received::<i32>(Times::Once).no_other_calls();
        }
    }

    mod return_where_constraint_tests {
        use super::*;

        #[test]
        fn return_where_constraint_Ok() {
            // Arrange
            let returned_number_value = 100;
            return_where_constraint::setup.returns(returned_number_value);
            let returned_string_value = String::from("quo vadis");
            return_where_constraint::setup.returns(returned_string_value.clone());

            // Act
            let actual_number_value = return_where_constraint();
            let actual_string_value: String = return_where_constraint();

            // Assert
            assert_eq!(returned_number_value, actual_number_value);
            return_where_constraint::received::<i32>(Times::Once).no_other_calls();

            assert_eq!(returned_string_value, actual_string_value);
            return_where_constraint::received::<String>(Times::Once).no_other_calls();

            return_where_constraint::received::<()>(Times::Never).no_other_calls();
        }

        #[test]
        fn return_constraint_CallBase_Ok() {
            // Arrange
            return_where_constraint::setup::<i32>().call_base();

            // Act
            let actual_number_value = return_where_constraint();

            // Assert
            assert_eq!(i32::default(), actual_number_value);
            return_where_constraint::received::<i32>(Times::Once).no_other_calls();
        }
    }

    mod get_return_different_tests {
        use super::*;

        #[test]
        fn get_return_different_Ok() {
            // Arrange
            let first_accepted_number = 10;
            let returned_string = String::from("quo vadis");
            get_return_different::setup::<i32, _>(Arg::Is(|x| *x == first_accepted_number))
                .returns(returned_string.clone());

            let second_accepted_number = 20;
            let returned_vec = vec![1, 23, 456];
            get_return_different::setup(second_accepted_number).returns(returned_vec.clone());

            // Act
            let actual_returned_string: String = get_return_different(first_accepted_number);
            let actual_returned_vec: Vec<i32> = get_return_different(second_accepted_number);

            // Assert
            assert_eq!(returned_string, actual_returned_string);
            assert_eq!(returned_vec, actual_returned_vec);
            get_return_different::received::<_, String>(first_accepted_number, Times::Once)
                .no_other_calls();
            get_return_different::received::<_, Vec<i32>>(second_accepted_number, Times::Once)
                .no_other_calls();
        }

        #[test]
        fn get_return_different_CallBase_Ok() {
            // Arrange
            let first_accepted_number = 10;
            get_return_different::setup::<i32, String>(Arg::Is(|x| *x == first_accepted_number))
                .call_base();

            let second_accepted_number = 20;
            get_return_different::setup::<i32, Vec<i32>>(second_accepted_number).call_base();

            // Act
            let actual_returned_string: String = get_return_different(first_accepted_number);
            let actual_returned_vec: Vec<i32> = get_return_different(second_accepted_number);

            // Assert
            assert_eq!(String::default(), actual_returned_string);
            assert_eq!(Vec::<i32>::default(), actual_returned_vec);
            get_return_different::received::<_, String>(first_accepted_number, Times::Once)
                .no_other_calls();
            get_return_different::received::<_, Vec<i32>>(second_accepted_number, Times::Once)
                .no_other_calls();
        }
    }
}

#[cfg(not(test))]
fn get_return<T>(value: T) -> T {
    return value;
}
#[cfg(test)]
pub use get_return::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod get_return {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct get_return_Call<'rs, T> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        _return_type: PhantomData<T>,
        _phantom_T: PhantomData<T>,
        value: T,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct get_return_ArgsChecker<'rs, T> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        _return_type: PhantomData<T>,
        _phantom_T: PhantomData<T>,
        value: Arg<'rs, T>,
    }
    impl<'rs, T> IArgsChecker for get_return_ArgsChecker<'rs, T> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &get_return_Call<'rs, T> = dyn_call.downcast_ref();
            vec![self.value.check("value", &call.value)]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct get_returnMockData<'rs, T> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        _phantom_T: PhantomData<T>,
        get_return_data: FnData<'rs, get_returnMock<'rs, T>, false>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct get_returnMockSetup<'rs, T> {
        data: Arc<get_returnMockData<'rs, T>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct get_returnMockReceived<'rs, T> {
        data: Arc<get_returnMockData<'rs, T>>,
    }
    #[doc(hidden)]
    pub struct get_returnMock<'rs, T> {
        pub setup: get_returnMockSetup<'rs, T>,
        pub received: get_returnMockReceived<'rs, T>,
        data: Arc<get_returnMockData<'rs, T>>,
    }
    impl<'rs, T> Default for get_returnMock<'rs, T> {
        fn default() -> Self {
            let data = Arc::new(get_returnMockData {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                get_return_data: FnData::new("get_return"),
            });
            return get_returnMock {
                setup: get_returnMockSetup { data: data.clone() },
                received: get_returnMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'rs, T> get_returnMockSetup<'rs, T> {
        pub fn setup<T>(
            &self,
            value: impl Into<Arg<'rs, T>>,
        ) -> FnTuner<'_, Self, (&T,), T, false> {
            let get_return_args_checker: get_return_ArgsChecker<'rs, T> = get_return_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _return_type: PhantomData,
                _phantom_T: PhantomData,
                value: value.into(),
            };
            let fn_tuner: FnTuner<'_, Self, (&T,), T, false> = self
                .data
                .get_return_data
                .add_config(get_return_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs, T> get_returnMockReceived<'rs, T> {
        pub fn received<T>(
            &self,
            value: impl Into<Arg<'rs, T>>,
            times: Times,
        ) -> FnVerifier<Self, (&'rs T,)> {
            let get_return_args_checker: get_return_ArgsChecker<'rs, T> = get_return_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _return_type: PhantomData,
                _phantom_T: PhantomData,
                value: value.into(),
            };
            self.data
                .get_return_data
                .verify_received(get_return_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'rs, T>(
        value: impl Into<Arg<'rs, T>>,
    ) -> FnTuner<'rs, get_returnMockSetup<'rs, T>, (&'rs T,), T, false> {
        let mock = get_global_mock::<get_returnMock<'rs, T>>();
        mock.data.get_return_data.reset();
        return mock.setup.setup(value);
    }
    pub fn received<'rs, T>(
        value: impl Into<Arg<'rs, T>>,
        times: Times,
    ) -> FnVerifier<get_returnMockReceived<'rs, T>, (&'rs T,)> {
        return get_global_mock::<get_returnMock<'rs, T>>()
            .received
            .clone()
            .received(value, times);
    }
    pub fn get_return<'__rsubstitute_arg_anonymous, 'rs, T>(value: T) -> T {
        let call: get_return_Call<'_, T> = unsafe {
            get_return_Call {
                _phantom_lifetime: PhantomData,
                _return_type: PhantomData,
                _phantom_T: PhantomData,
                value: core::mem::transmute(value),
            }
        };
        let mock = get_global_mock::<get_returnMock<'rs, T>>();
        return mock.data.get_return_data.handle_returning(call);
    }
}
