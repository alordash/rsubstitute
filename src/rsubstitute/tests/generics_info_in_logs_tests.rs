pub use __rsubstitute_generated_TraitMock::{Trait, TraitMock};
#[allow(non_camel_case_types)]
mod __rsubstitute_generated_TraitMock {
    #[allow(unused_imports)]
    use super::*;
    use rsubstitute::for_generated::*;
    pub trait Trait<'a, T1, const B: bool> {
        fn work<'b, T2, const N: usize>(&self, v: &'b T1) -> T2;
    }
    pub struct work_Call<'b, 'a, T2, const N: usize, T1, const B: bool> {
        generics: PhantomData<(T2, &'b (), &'a (), T1, &'b T1)>,
        v: *const T1,
    }
    impl<'b, 'a, T2, const N: usize, T1, const B: bool> IGenericsInfoProvider
        for work_Call<'b, 'a, T2, N, T1, B>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![
                // TODO - this should not include merged generics, only ones from source signature
                generic_type_info("T2", core::any::type_name::<T2>()),
                generic_const_info("N", N),
                // generic_type_info("T1", core::any::type_name::<T1>()),
                // generic_const_info("B", B),
            ]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            [tid::<T2>(), tid::<T1>()];
        }
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            const_hash(&N, hasher);
            const_hash(&B, hasher);
        }
    }
    impl<'b, 'a, T2, const N: usize, T1, const B: bool> ICall for work_Call<'b, 'a, T2, N, T1, B> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "v",
                &self.v,
                (&ArgPrinter(transmute_lifetime!(&self.v, &&'b T1))).debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.v,))) as *mut _ as *mut ()
        }
    }
    struct work_ArgsChecker<'b, 'a, T2, const N: usize, T1, const B: bool> {
        generics: PhantomData<(T2, &'b (), &'a (), T1, &'b T1)>,
        v: Arg<*const T1>,
    }
    impl<'b, 'a, T2, const N: usize, T1, const B: bool> IGenericsInfoProvider
        for work_ArgsChecker<'b, 'a, T2, N, T1, B>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![
                generic_type_info("T2", core::any::type_name::<T2>()),
                generic_const_info("N", N),
                // generic_type_info("T1", core::any::type_name::<T1>()),
                // generic_const_info("B", B),
            ]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            [tid::<T2>(), tid::<T1>()];
        }
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            const_hash(&N, hasher);
            const_hash(&B, hasher);
        }
    }
    impl<'b, 'a, T2, const N: usize, T1, const B: bool> IArgsChecker
        for work_ArgsChecker<'b, 'a, T2, N, T1, B>
    {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &work_Call<'b, 'a, T2, N, T1, B> = dyn_call.downcast_ref();
            vec![transmute_lifetime!(&self.v, &Arg<&'b T1>).check_ref(
                "v",
                transmute_lifetime!(&call.v),
                (&ArgPrinter(transmute_lifetime!(&call.v, &&'b T1))).debug_string(),
            )]
        }
        fn fmt_args(&self) -> String {
            format!(
                "{}",
                (&ArgPrinter(transmute_lifetime!(&&self.v, &&Arg<&'b T1>))).debug_string()
            )
        }
    }
    pub struct TraitMock<'a, T1, const B: bool> {
        pub data: ::rsubstitute::for_generated::SharedMockData,
        generics: PhantomData<(&'a (), T1)>,
    }
    impl<'a, T1, const B: bool> Trait<'a, T1, B> for TraitMock<'a, T1, B> {
        fn work<'b, T2, const N: usize>(&self, v: &'b T1) -> T2 {
            let fn_data: &FnData<TraitMock<'a, T1, B>, true, false, false> =
                self.data.get_shared_fn_data("work");
            fn_data.handle(
                &TraitMock::<'a, T1, B> {
                    data: self.data.clone(),
                    generics: PhantomData,
                },
                work_Call::<'b, 'a, T2, N, T1, B> {
                    generics: ::core::marker::PhantomData,
                    v: transmute_lifetime!(v),
                },
            )
        }
    }
    impl<'a, T1, const B: bool> TraitMock<'a, T1, B> {
        pub fn new() -> Self {
            Self {
                data: ::core::default::Default::default(),
                generics: PhantomData,
            }
        }
        pub fn setup(&mut self) -> TraitSetup<'a, T1, B> {
            TraitSetup::<'a, T1, B> {
                data: self.data.clone(),
                generics: PhantomData,
            }
        }
        pub fn received(&mut self) -> TraitReceived<'a, T1, B> {
            TraitReceived::<'a, T1, B> {
                // g: PhantomData,
                // data: self.data.clone(),
                data: self.data.clone(),
                generics: PhantomData,
            }
        }
    }
    pub struct TraitSetup<'a, T1, const B: bool> {
        data: ::rsubstitute::for_generated::SharedMockData,
        generics: PhantomData<(&'a (), T1)>,
    }
    impl<'a, T1, const B: bool> TraitSetup<'a, T1, B> {
        pub fn work<'__rsa, 'b, T2, const N: usize>(
            &self,
            v: impl Into<Arg<&'b T1>>,
        ) -> FnConfigurator<
            '_,
            TraitMock<'a, T1, B>,
            Self,
            (&'__rsa &'b T1,),
            T2,
            TraitMock<'a, T1, B>,
            true,
            false,
            false,
        > {
            let fn_data: &FnData<TraitMock<'a, T1, B>, true, false, false> =
                self.data.get_shared_fn_data("work");
            let args_checker = work_ArgsChecker::<'b, 'a, T2, N, T1, B> {
                generics: ::core::marker::PhantomData,
                v: transmute_lifetime!(v.into()),
            };
            let fn_configurator: FnConfigurator<
                '_,
                TraitMock<'a, T1, B>,
                Self,
                (&'__rsa &'b T1,),
                T2,
                TraitMock<'a, T1, B>,
                true,
                false,
                false,
            > = fn_data.add_config(args_checker, self);
            transmute_lifetime!(fn_configurator)
        }
    }
    pub struct TraitReceived<'a, T1, const B: bool> {
        data: ::rsubstitute::for_generated::SharedMockData,
        generics: PhantomData<(&'a (), T1)>,
    }
    impl<'a, T1, const B: bool> Clone for TraitReceived<'a, T1, B> {
        fn clone(&self) -> Self {
            Self {
                data: self.data.clone(),
                generics: PhantomData,
            }
        }
    }
    impl<'a, T1, const B: bool> TraitReceived<'a, T1, B> {
        pub fn work<'__rsa, 'b, T2, const N: usize>(
            &self,
            v: impl Into<Arg<&'b T1>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rsa &'b T1,)>
        where
            'b: '__rsa,
            'a: '__rsa,
            '__rsa: 'b + 'a,
        {
            let fn_data: &FnData<TraitMock<'a, T1, B>, true, false, false> =
                self.data.get_shared_fn_data("work");
            let args_checker = work_ArgsChecker::<'b, 'a, T2, N, T1, B> {
                generics: ::core::marker::PhantomData,
                v: transmute_lifetime!(v.into()),
            };
            fn_data.verify_received(args_checker, times);
            FnVerifier::new(self.clone())
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else(["work"])
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;
    use rsubstitute_core::Times;

    #[test]
    fn work_NoConfigs_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 5>(&14));

        // Assert
        let expected_panic_msg = "Mock wasn't configured to handle following call:
	work<f32, 5>(14)";
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_OnlyUnsuitableConfigs_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let value = 5;
        mock.setup().work::<f32, 1>(&2).returns(3.0f32);
        mock.setup().work::<f32, 10>(&value).returns(3.0f32);
        mock.setup()
            .work::<[u8; 4], 10>(&value)
            .returns([1, 2, 3, 4]);
        mock.setup().work::<[u8; 4], 10>(&2).returns([1, 2, 3, 4]);

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 1>(&value));

        // Assert
        let expected_panic_msg = "Mock wasn't configured to handle following call because no return value was provided:
	work<f32, 1>(5)
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 0/1 arguments: work(*5*)";
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_NoReturnValue_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup().work::<f32, 1>(&value);
        mock.setup().work::<f32, 1>(&value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);
        mock.received()
            .work::<f32, 1>(&value, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn work_DidNotReceiveSameGenerics_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let actual_value = 5;
        let expected_value = actual_value + 1;
        let returned_value = 3.0f32;
        const N: usize = 1;
        mock.setup()
            .work::<f32, N>(&actual_value)
            .returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&actual_value);
        let panic_msg =
            record_panic(|| mock.received().work::<f32, 1>(&expected_value, Times::Once));

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let actual_value_ptr = core::ptr::from_ref(&actual_value);
        let expected_value_ptr = core::ptr::from_ref(&expected_value);
        let expected_panic_msg = format!(
            "Expected to receive a call exactly once matching:
	work<f32, {N}>((&i32): equal to {expected_value})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
work(*5*)
	1. v (&i32):
		Expected reference (ptr: {expected_value_ptr:?}): 6
		Actual reference   (ptr: {actual_value_ptr:?}): 5"
        );

        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn work_DidNotReceiveDifferentGenerics_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup().work::<f32, 1>(&value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(&value);
        let panic_msg = record_panic(|| mock.received().work::<String, 124>(&value, Times::Once));

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let expected_panic_msg = "Expected to receive a call exactly once matching:
	work<alloc::string::String, 124>((&i32): equal to 5)
Actually received no matching calls
Received no non-matching calls";
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }

    #[test]
    fn work_ReceivedUnexpectedCalls_Ok() {
        // Arrange
        let mut mock = TraitMock::<i32, true>::new();

        let first_value = 5;
        let first_returned_value = 3.0f32;
        const FIRST_N: usize = 1;
        let second_value = 100;
        let second_returned_value = [4; 3];
        const SECOND_N: usize = 200;
        mock.setup()
            .work::<f32, FIRST_N>(&first_value)
            .returns(first_returned_value);
        mock.setup()
            .work::<_, SECOND_N>(&second_value)
            .returns(second_returned_value);

        // Act
        let actual_first_returned_value = mock.work::<f32, FIRST_N>(&first_value);
        let actual_second_returned_value = mock.work::<[i32; 3], SECOND_N>(&second_value);
        let panic_msg = record_panic(|| mock.received().no_other_calls());

        // Assert
        assert_eq!(first_returned_value, actual_first_returned_value);
        assert_eq!(second_returned_value, actual_second_returned_value);

        let expected_panic_msg =
            "Did not expect to receive any other calls. Received 2 unexpected calls:
1. work<f32, 1>(5)
2. work<[i32; 3], 200>(100)";
        assert_eq!(Some(expected_panic_msg.to_owned()), panic_msg);
    }
}
