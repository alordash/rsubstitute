use rsubstitute::macros::mock;

#[mock]
trait Trait {
    fn work(&self, v1: i32, v2: i32, v3: i32, v4: i32) -> i32;
}

// #[mock]
// fn work(_v1: i32, _v2: i32, _v3: i32, _v4: i32) -> i32 {
//     1
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::Arg;

    #[test]
    fn trait_work_Panics() {
        // Arrange
        let mock = TraitMock::new();
        let (v1, v2, v3, v4) = (10, 20, 30, 40);
        mock.setup
            .work(Arg::Any, Arg::Any, Arg::Any, Arg::is(|_| false))
            .returns(1)
            .work(Arg::Any, Arg::Any, Arg::is(|_| false), Arg::is(|_| false))
            .returns(1)
            .work(
                Arg::Any,
                Arg::is(|_| false),
                Arg::is(|_| false),
                Arg::is(|_| false),
            )
            .returns(1)
            .work(
                Arg::is(|_| false),
                Arg::is(|_| false),
                Arg::is(|_| false),
                Arg::is(|_| false),
            )
            .returns(1);

        // Act
        let actual_error_msg = record_panic(|| mock.work(v1, v2, v3, v4));

        // Assert
        let expected_error_msg = format!("Mock wasn't configured to handle following call:
	Trait::work({v1}, {v2}, {v3}, {v4})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 3/4 arguments: Trait::work({v1}, {v2}, {v3}, *{v4}*)
	2. Matched 2/4 arguments: Trait::work({v1}, {v2}, *{v3}*, *{v4}*)
	3. Matched 1/4 arguments: Trait::work({v1}, *{v2}*, *{v3}*, *{v4}*)
	4. Matched 0/4 arguments: Trait::work(*{v1}*, *{v2}*, *{v3}*, *{v4}*)");
        assert_eq!(expected_error_msg, actual_error_msg);
    }

    #[test]
    fn fn_work_Panics() {
        // Arrange
        let (v1, v2, v3, v4) = (10, 20, 30, 40);
        work::setup(Arg::Any, Arg::Any, Arg::Any, Arg::is(|_| false))
            .returns(1)
            .setup(Arg::Any, Arg::Any, Arg::is(|_| false), Arg::is(|_| false))
            .returns(1)
            .setup(
                Arg::Any,
                Arg::is(|_| false),
                Arg::is(|_| false),
                Arg::is(|_| false),
            )
            .returns(1)
            .setup(
                Arg::is(|_| false),
                Arg::is(|_| false),
                Arg::is(|_| false),
                Arg::is(|_| false),
            )
            .returns(1);

        // Act
        let actual_error_msg = record_panic(|| work(v1, v2, v3, v4));

        // Assert
        let expected_error_msg = format!("Mock wasn't configured to handle following call:
	work({v1}, {v2}, {v3}, {v4})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 3/4 arguments: work({v1}, {v2}, {v3}, *{v4}*)
	2. Matched 2/4 arguments: work({v1}, {v2}, *{v3}*, *{v4}*)
	3. Matched 1/4 arguments: work({v1}, *{v2}*, *{v3}*, *{v4}*)
	4. Matched 0/4 arguments: work(*{v1}*, *{v2}*, *{v3}*, *{v4}*)");
        assert_eq!(expected_error_msg, actual_error_msg);
    }
}

#[cfg(not(test))]
fn work(_v1: i32, _v2: i32, _v3: i32, _v4: i32) -> i32 {
    1
}
#[cfg(test)]
pub use work::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod work {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    pub struct work_Call {
        _phantom__v1: PhantomData<i32>,
        _phantom__v2: PhantomData<i32>,
        _phantom__v3: PhantomData<i32>,
        _phantom__v4: PhantomData<i32>,
        _v1: i32,
        _v2: i32,
        _v3: i32,
        _v4: i32,
    }
    impl IArgsInfosProvider for work_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new("_v1", &self._v1, (&ArgPrinter(&self._v1)).debug_string()),
                ArgInfo::new("_v2", &self._v2, (&ArgPrinter(&self._v2)).debug_string()),
                ArgInfo::new("_v3", &self._v3, (&ArgPrinter(&self._v3)).debug_string()),
                ArgInfo::new("_v4", &self._v4, (&ArgPrinter(&self._v4)).debug_string()),
            ]
        }
    }
    impl IArgsTupleProvider for work_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self._v1, &self._v2, &self._v3, &self._v4))) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for work_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct work_ArgsChecker {
        _phantom__v1: PhantomData<i32>,
        _phantom__v2: PhantomData<i32>,
        _phantom__v3: PhantomData<i32>,
        _phantom__v4: PhantomData<i32>,
        _v1: Arg<i32>,
        _v2: Arg<i32>,
        _v3: Arg<i32>,
        _v4: Arg<i32>,
    }
    impl IArgsChecker for work_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &work_Call = dyn_call.downcast_ref();
            vec![
                self._v1.check(
                    "_v1",
                    transmute_lifetime!(&call._v1),
                    (&ArgPrinter(&call._v1)).debug_string(),
                ),
                self._v2.check(
                    "_v2",
                    transmute_lifetime!(&call._v2),
                    (&ArgPrinter(&call._v2)).debug_string(),
                ),
                self._v3.check(
                    "_v3",
                    transmute_lifetime!(&call._v3),
                    (&ArgPrinter(&call._v3)).debug_string(),
                ),
                self._v4.check(
                    "_v4",
                    transmute_lifetime!(&call._v4),
                    (&ArgPrinter(&call._v4)).debug_string(),
                ),
            ]
        }
    }
    impl IArgsFormatter for work_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!(
                "{}, {}, {}, {}",
                (&ArgPrinter(&self._v1)).debug_string(),
                (&ArgPrinter(&self._v2)).debug_string(),
                (&ArgPrinter(&self._v3)).debug_string(),
                (&ArgPrinter(&self._v4)).debug_string()
            )
        }
    }
    impl IGenericsInfoProvider for work_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct workMockData {
        pub work: FnData<'static, workMock, false, false>,
    }
    #[doc(hidden)]
    pub struct workMockSetup {
        data: Arc<workMockData>,
    }
    impl Clone for workMockSetup {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct workMockReceived {
        data: Arc<workMockData>,
    }
    impl Clone for workMockReceived {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct workMock {
        pub setup: workMockSetup,
        pub received: workMockReceived,
        pub data: Arc<workMockData>,
    }
    impl AsRef<workMock> for workMock {
        fn as_ref(&self) -> &workMock {
            self
        }
    }
    impl Clone for workMock {
        fn clone(&self) -> Self {
            Self {
                setup: (&self.setup).clone(),
                received: (&self.received).clone(),
                data: (&self.data).clone(),
            }
        }
    }
    impl Default for workMock {
        fn default() -> Self {
            let data = Arc::new(workMockData {
                work: FnData::new("work"),
            });
            return workMock {
                setup: workMockSetup { data: data.clone() },
                received: workMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl workMockSetup {
        pub fn setup<'__rsa>(
            &self,
            _v1: impl Into<Arg<i32>>,
            _v2: impl Into<Arg<i32>>,
            _v3: impl Into<Arg<i32>>,
            _v4: impl Into<Arg<i32>>,
        ) -> FnTuner<'_, workMock, Self, (&i32, &i32, &i32, &i32), i32, workMock, false, false>
        {
            let work_args_checker: work_ArgsChecker = work_ArgsChecker {
                _phantom__v1: PhantomData,
                _phantom__v2: PhantomData,
                _phantom__v3: PhantomData,
                _phantom__v4: PhantomData,
                _v1: transmute_lifetime!(_v1.into()),
                _v2: transmute_lifetime!(_v2.into()),
                _v3: transmute_lifetime!(_v3.into()),
                _v4: transmute_lifetime!(_v4.into()),
            };
            let fn_tuner: FnTuner<
                '_,
                workMock,
                Self,
                (&i32, &i32, &i32, &i32),
                i32,
                workMock,
                false,
                false,
            > = self.data.work.add_config(work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl workMockReceived {
        pub fn received<'__rsa>(
            &self,
            _v1: impl Into<Arg<i32>>,
            _v2: impl Into<Arg<i32>>,
            _v3: impl Into<Arg<i32>>,
            _v4: impl Into<Arg<i32>>,
            times: Times,
        ) -> FnVerifier<Self, (&i32, &i32, &i32, &i32)> {
            let work_args_checker: work_ArgsChecker = work_ArgsChecker {
                _phantom__v1: PhantomData,
                _phantom__v2: PhantomData,
                _phantom__v3: PhantomData,
                _phantom__v4: PhantomData,
                _v1: transmute_lifetime!(_v1.into()),
                _v2: transmute_lifetime!(_v2.into()),
                _v3: transmute_lifetime!(_v3.into()),
                _v4: transmute_lifetime!(_v4.into()),
            };
            self.data.work.verify_received(work_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn get_mock<'__rsa>() -> &'__rsa workMock {
        get_global_mock::<workMock>()
    }
    pub fn setup<'__rsa>(
        _v1: impl Into<Arg<i32>>,
        _v2: impl Into<Arg<i32>>,
        _v3: impl Into<Arg<i32>>,
        _v4: impl Into<Arg<i32>>,
    ) -> FnTuner<
        '__rsa,
        workMock,
        workMockSetup,
        (&'__rsa i32, &'__rsa i32, &'__rsa i32, &'__rsa i32),
        i32,
        workMock,
        false,
        false,
    > {
        let mock = get_mock();
        mock.data.work.reset();
        return mock.setup.setup(_v1, _v2, _v3, _v4);
    }
    pub fn received<'__rsa>(
        _v1: impl Into<Arg<i32>>,
        _v2: impl Into<Arg<i32>>,
        _v3: impl Into<Arg<i32>>,
        _v4: impl Into<Arg<i32>>,
        times: Times,
    ) -> FnVerifier<workMockReceived, (&i32, &i32, &i32, &i32)> {
        return get_mock()
            .received
            .clone()
            .received(_v1, _v2, _v3, _v4, times);
    }
    pub fn work(_v1: i32, _v2: i32, _v3: i32, _v4: i32) -> i32 {
        let call: work_Call = work_Call {
            _phantom__v1: PhantomData,
            _phantom__v2: PhantomData,
            _phantom__v3: PhantomData,
            _phantom__v4: PhantomData,
            _v1: transmute_lifetime!(_v1),
            _v2: transmute_lifetime!(_v2),
            _v3: transmute_lifetime!(_v3),
            _v4: transmute_lifetime!(_v4),
        };
        let mock = get_mock::<'_>();
        return mock.data.clone().work.handle_returning(mock, call);
    }
}
