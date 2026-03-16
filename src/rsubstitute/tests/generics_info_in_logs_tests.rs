trait Trait<T1, const B: bool> {
    fn work<T2, const N: usize>(&self, v: T1) -> T2;
}
#[cfg(test)]
pub use __rsubstitute_generated_Trait::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Trait {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider)]
    pub struct work_Call<'__rs, T1, const B: bool, T2, const N: usize> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_T1: PhantomData<T1>,
        _phantom_T2: PhantomData<T2>,
        _return_type: PhantomData<T2>,
        v: T1,
    }
    impl<'__rs, T1, const B: bool, T2, const N: usize> IGenericsInfoProvider
        for work_Call<'__rs, T1, B, T2, N>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![
                generic_type_info("T1", core::any::TypeId::of::<T1>()),
                generic_const_info("B", B),
                generic_type_info("T2", core::any::type_name::<T2>()),
                generic_const_info("N", N),
            ]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<T1>(), tid::<T2>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {
            const_hash(&B, hasher);
            const_hash(&N, hasher);
        }
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct work_ArgsChecker<'__rs, T1, const B: bool, T2, const N: usize> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_T1: PhantomData<T1>,
        _phantom_T2: PhantomData<T2>,
        _return_type: PhantomData<T2>,
        v: Arg<'__rs, T1>,
    }
    impl<'__rs, T1, const B: bool, T2, const N: usize> IArgsChecker
        for work_ArgsChecker<'__rs, T1, B, T2, N>
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &work_Call<'__rs, T1, B, T2, N> = dyn_call.downcast_ref();
            vec![
                self.v
                    .check("v", &call.v, (&ArgPrinter(&&call.v)).debug_string()),
            ]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<'__rs, T1, const B: bool> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_T1: PhantomData<T1>,
        pub work: FnData<'static, TraitMock<'__rs, T1, B>, false, false>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitMockSetup<'__rs, T1, const B: bool> {
        data: Arc<TraitMockData<'__rs, T1, B>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitMockReceived<'__rs, T1, const B: bool> {
        data: Arc<TraitMockData<'__rs, T1, B>>,
    }
    #[derive(CloneForRSubstitute)]
    pub struct TraitMock<'__rs, T1, const B: bool> {
        pub setup: TraitMockSetup<'__rs, T1, B>,
        pub received: TraitMockReceived<'__rs, T1, B>,
        pub data: Arc<TraitMockData<'__rs, T1, B>>,
    }
    impl<'__rs, T1, const B: bool> Trait<T1, B> for TraitMock<'__rs, T1, B> {
        fn work<T2, const N: usize>(&self, v: T1) -> T2 {
            let call: work_Call<'_, T1, B, T2, N> = work_Call {
                _phantom_lifetime: PhantomData,
                _phantom_T1: PhantomData,
                _phantom_T2: PhantomData,
                _return_type: PhantomData,
                v: transmute_lifetime!(v),
            };
            return self.data.work.handle_returning(&self, call);
        }
    }
    impl<'__rs, T1, const B: bool> TraitMock<'__rs, T1, B> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_lifetime: PhantomData,
                _phantom_T1: PhantomData,
                work: FnData::new("work"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rs, T1, const B: bool> TraitMockSetup<'__rs, T1, B> {
        pub fn work<'__rsa, T2, const N: usize>(
            &self,
            v: impl Into<Arg<'__rsa, T1>>,
        ) -> FnTuner<'_, TraitMock<'__rs, T1, B>, Self, (&'__rs T1,), T2, false, false> {
            let work_args_checker: work_ArgsChecker<'_, T1, B, T2, N> = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_T1: PhantomData,
                _phantom_T2: PhantomData,
                _return_type: PhantomData,
                v: transmute_lifetime!(v.into()),
            };
            let fn_tuner: FnTuner<
                '_,
                TraitMock<'__rs, T1, B>,
                Self,
                (&'__rs T1,),
                T2,
                false,
                false,
            > = self.data.work.add_config(work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'__rs, T1, const B: bool> TraitMockReceived<'__rs, T1, B> {
        pub fn work<'__rsa, T2, const N: usize>(
            &self,
            v: impl Into<Arg<'__rsa, T1>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rs T1,)> {
            let work_args_checker: work_ArgsChecker<'_, T1, B, T2, N> = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_T1: PhantomData,
                _phantom_T2: PhantomData,
                _return_type: PhantomData,
                v: transmute_lifetime!(v.into()),
            };
            self.data.work.verify_received(work_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute_core::Times;

    #[test]
    fn work_NoConfigs_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 5>(14));

        // Assert
        panic!("{panic_msg}");
    }

    #[test]
    fn work_OnlyUnsuitableConfigs_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        mock.setup.work::<f32, 1>(2).returns(3.0f32);
        mock.setup.work::<f32, 10>(value).returns(3.0f32);
        mock.setup.work::<[u8; 4], 10>(value).returns([1, 2, 3, 4]);
        mock.setup.work::<[u8; 4], 10>(2).returns([1, 2, 3, 4]);

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 1>(value));

        // Assert
        panic!("{panic_msg}")
    }

    #[test]
    fn work_NoReturnValue_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        mock.setup.work::<f32, 1>(value);
        mock.setup.work::<f32, 1>(value).returns(3.0f32);

        // Act
        let panic_msg = record_panic(|| mock.work::<f32, 1>(value));

        // Assert
        panic!("{panic_msg}")
    }

    #[test]
    fn work_DidNotReceiveSameGenerics_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup.work::<f32, 1>(value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let panic_msg = record_panic(|| mock.received.work::<f32, 1>(value + 1, Times::Once));

        panic!("{panic_msg}")
    }

    #[test]
    fn work_DidNotReceiveDifferentGenerics_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup.work::<f32, 1>(value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let panic_msg = record_panic(|| mock.received.work::<String, 124>(value, Times::Once));

        panic!("{panic_msg}")
    }

    #[test]
    fn work_ReceivedUnexpectedCalls_Ok() {
        // Arrange
        let mock = TraitMock::<i32, true>::new();

        let value = 5;
        let returned_value = 3.0f32;
        mock.setup.work::<f32, 1>(value).returns(returned_value);

        // Act
        let actual_returned_value = mock.work::<f32, 1>(value);

        // Assert
        assert_eq!(returned_value, actual_returned_value);

        let panic_msg = record_panic(|| mock.received.no_other_calls());

        panic!("{panic_msg}")
    }
}
