use rsubstitute::macros::mock;
use std::fmt::{Debug, Display};
#[cfg(not(test))]
fn get_return<T>(value: T) -> T {
    return value;
}
#[cfg(test)]
pub use get_return::*;
#[cfg(test)]
#[allow(dead_code)]
#[allow(unused)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod get_return {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct get_return_Call<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        _phantom_T: PhantomData<T>,
        value: T,
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > IArgInfosProvider for get_return_Call<'__rsubstitute_arg_field_lifetime, T>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new("value", self.value.clone())]
        }
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter)]
    pub struct get_return_ArgsChecker<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        _phantom_T: PhantomData<T>,
        value: Arg<T>,
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > IArgsChecker<get_return_Call<'__rsubstitute_arg_field_lifetime, T>>
        for get_return_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>
    {
        fn check(
            &self,
            call: &get_return_Call<'__rsubstitute_arg_field_lifetime, T>,
        ) -> Vec<ArgCheckResult> {
            vec![self.value.check("value", &call.value)]
        }
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > IBaseCaller<get_return_Call<'__rsubstitute_arg_field_lifetime, T>, T>
        for get_returnMock<'__rsubstitute_arg_field_lifetime, T>
    {
        fn call_base(&self, call: get_return_Call<'__rsubstitute_arg_field_lifetime, T>) -> T {
            let get_return_Call { value: value, .. } = call;
            return value;
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct get_returnMockData<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        get_return_data: FnData<
            get_returnMock<'__rsubstitute_arg_field_lifetime, T>,
            get_return_Call<'__rsubstitute_arg_field_lifetime, T>,
            get_return_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>,
            T,
        >,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct get_returnMockSetup<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        data: Arc<get_returnMockData<'__rsubstitute_arg_field_lifetime, T>>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct get_returnMockReceived<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        data: Arc<get_returnMockData<'__rsubstitute_arg_field_lifetime, T>>,
    }
    #[doc(hidden)]
    pub struct get_returnMock<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        setup: get_returnMockSetup<'__rsubstitute_arg_field_lifetime, T>,
        received: get_returnMockReceived<'__rsubstitute_arg_field_lifetime, T>,
        data: Arc<get_returnMockData<'__rsubstitute_arg_field_lifetime, T>>,
    }
    unsafe impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > Send for get_returnMock<'__rsubstitute_arg_field_lifetime, T>
    {
    }
    unsafe impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > Sync for get_returnMock<'__rsubstitute_arg_field_lifetime, T>
    {
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > Default for get_returnMock<'__rsubstitute_arg_field_lifetime, T>
    {
        fn default() -> Self {
            let data = Arc::new(get_returnMockData {
                _phantom_lifetime: PhantomData,
                get_return_data: FnData::new("get_return", &SERVICES),
            });
            return get_returnMock {
                setup: get_returnMockSetup { data: data.clone() },
                received: get_returnMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > get_returnMockSetup<'__rsubstitute_arg_field_lifetime, T>
    {
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
            value: impl Into<Arg<T>>,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            get_returnMock<'__rsubstitute_arg_field_lifetime, T>,
            get_return_Call<'__rsubstitute_arg_field_lifetime, T>,
            get_return_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>,
            T,
            Self,
        > {
            let get_return_args_checker = get_return_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: value.into(),
            };
            let fn_config = self
                .data
                .get_return_data
                .add_config(get_return_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > get_returnMockReceived<'__rsubstitute_arg_field_lifetime, T>
    {
        pub fn received(self, value: impl Into<Arg<T>>, times: Times) -> Self {
            let get_return_args_checker = get_return_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: value.into(),
            };
            self.data
                .get_return_data
                .verify_received(get_return_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    >(
        value: impl Into<Arg<T>>,
    ) -> SharedFnConfig<
        '__rsubstitute_arg_field_lifetime,
        get_returnMock<'__rsubstitute_arg_field_lifetime, T>,
        get_return_Call<'__rsubstitute_arg_field_lifetime, T>,
        get_return_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>,
        T,
        get_returnMockSetup<'__rsubstitute_arg_field_lifetime, T>,
    > {
        let mock = get_global_mock::<get_returnMock<'__rsubstitute_arg_field_lifetime, T>>();
        mock.data.get_return_data.reset();
        return mock.setup.setup(value);
    }
    pub fn received<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    >(
        value: impl Into<Arg<T>>,
        times: Times,
    ) -> get_returnMockReceived<'__rsubstitute_arg_field_lifetime, T> {
        return get_global_mock::<get_returnMock<'__rsubstitute_arg_field_lifetime, T>>()
            .received
            .clone()
            .received(value, times);
    }
    pub fn get_return<
        '__rsubstitute_arg_anonymous,
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    >(
        value: T,
    ) -> T {
        let call = unsafe {
            get_return_Call {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: std::mem::transmute(value),
            }
        };
        let mock = get_global_mock::<get_returnMock<'__rsubstitute_arg_field_lifetime, T>>();
        return mock.data.get_return_data.handle_base_returning(&mock, call);
    }
}

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
            return_constraint::setup().returns(returned_number_value);
            let returned_string_value = String::from("quo vadis");
            return_constraint::setup().returns(returned_string_value.clone());

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
            return_where_constraint::setup().returns(returned_number_value);
            let returned_string_value = String::from("quo vadis");
            return_where_constraint::setup().returns(returned_string_value.clone());

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
