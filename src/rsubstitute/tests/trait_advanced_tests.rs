use rsubstitute::macros::mock;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub numbers: Vec<i32>,
}

static mut BASE_MUT_REF: i32 = 10;

// #[mock(base)]
// trait Trait<'a, 'b> {
//     fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32;
//
//     fn return_mut_ref(&self) -> &'a mut i32;
//
//     fn return_mut_ref_with_base(&self) -> &'a mut i32 {
//         unsafe { &mut *&raw mut BASE_MUT_REF }
//     }
//
//     fn foo_sum(&mut self, Foo { mut numbers }: Foo) -> i32 {
//         for number in numbers.iter_mut() {
//             *number *= 2;
//         }
//         return numbers.into_iter().sum();
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    #[test]
    fn accept_ref_Ok() {
        // Arrange
        let trait_mock = TraitMock::new();
        let v = &&&&32;
        let r = &&&&8;
        trait_mock.setup.accept_ref(v).returns(r);

        // Act
        let actual_r = trait_mock.accept_ref(v);

        // Assert
        assert_eq!(r, actual_r);

        trait_mock
            .received
            .accept_ref(v, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn return_mut_ref_Ok() {
        // Arrange
        let trait_mock = TraitMock::new();
        let r = &mut 5;
        trait_mock.setup.return_mut_ref().returns(r);

        // Act
        let actual_r = trait_mock.return_mut_ref();

        // Assert
        assert_eq!(r, actual_r);

        trait_mock
            .received
            .return_mut_ref(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn return_mut_ref_with_base_Ok() {
        // Arrange
        let trait_mock = TraitMock::new();
        let r = &mut 5;
        trait_mock.setup.return_mut_ref_with_base().returns(r);

        // Act
        let actual_r = trait_mock.return_mut_ref_with_base();

        // Assert
        assert_eq!(r, actual_r);

        trait_mock
            .received
            .return_mut_ref_with_base(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn return_mut_ref_with_base_CallBase_Ok() {
        // Arrange
        let trait_mock = TraitMock::new();
        trait_mock.setup.return_mut_ref_with_base().call_base();

        // Act
        let actual_r = trait_mock.return_mut_ref_with_base();

        // Assert
        unsafe {
            assert_eq!(&*&raw mut BASE_MUT_REF, actual_r);
        }
        trait_mock
            .received
            .return_mut_ref_with_base(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn foo_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let v = Foo {
            numbers: vec![5, 55],
        };
        let r = 42;
        trait_mock.setup.foo_sum(v.clone()).returns(r);

        // Act
        let actual_r = trait_mock.foo_sum(v.clone());

        // Assert
        assert_eq!(r, actual_r);

        trait_mock.received.foo_sum(v, Times::Once).no_other_calls();
    }

    #[test]
    fn foo_CallBase_Ok() {
        // Arrange
        let mut trait_mock = TraitMock::new();
        let v = Foo {
            numbers: vec![5, 55],
        };
        trait_mock.setup.foo_sum(v.clone()).call_base();

        // Act
        let actual_r = trait_mock.foo_sum(v.clone());

        // Assert
        let expected_r = v.numbers.iter().sum::<i32>() * 2;
        assert_eq!(expected_r, actual_r);

        trait_mock.received.foo_sum(v, Times::Once).no_other_calls();
    }
}

trait Trait<'a, 'b> {
    fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32;

    fn return_mut_ref(&self) -> &'a mut i32;

    fn return_mut_ref_with_base(&self) -> &'a mut i32 {
        unsafe { &mut *&raw mut BASE_MUT_REF }
    }

    fn foo_sum(&mut self, Foo { mut numbers }: Foo) -> i32 {
        for number in numbers.iter_mut() {
            *number *= 2;
        }
        return numbers.into_iter().sum();
    }
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
    #[derive(
        IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider, CloneForRSubstitute,
    )]
    pub struct accept_ref_Call<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        r: &'a &'b &'a &'__rs i32,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct accept_ref_ArgsChecker<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        r: Arg<'__rs, &'a &'b &'a &'__rs i32>,
    }
    impl<'__rs, 'a, 'b> IArgsChecker for accept_ref_ArgsChecker<'__rs, 'a, 'b> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &accept_ref_Call<'__rs, 'a, 'b> = dyn_call.downcast_ref();
            vec![
                self.r
                    .check_ref("r", &call.r, (&ArgPrinter(&&call.r)).debug_string()),
            ]
        }
    }
    #[doc(hidden)]
    #[derive(
        IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider, CloneForRSubstitute,
    )]
    pub struct return_mut_ref_Call<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct return_mut_ref_ArgsChecker<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
    }
    impl<'__rs, 'a, 'b> IArgsChecker for return_mut_ref_ArgsChecker<'__rs, 'a, 'b> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &return_mut_ref_Call<'__rs, 'a, 'b> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(
        IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider, CloneForRSubstitute,
    )]
    pub struct return_mut_ref_with_base_Call<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct return_mut_ref_with_base_ArgsChecker<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
    }
    impl<'__rs, 'a, 'b> IArgsChecker for return_mut_ref_with_base_ArgsChecker<'__rs, 'a, 'b> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &return_mut_ref_with_base_Call<'__rs, 'a, 'b> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(
        IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider, CloneForRSubstitute,
    )]
    pub struct foo_sum_Call<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        arg_1: Foo,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct foo_sum_ArgsChecker<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        arg_1: Arg<'__rs, Foo>,
    }
    impl<'__rs, 'a, 'b> IArgsChecker for foo_sum_ArgsChecker<'__rs, 'a, 'b> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &foo_sum_Call<'__rs, 'a, 'b> = dyn_call.downcast_ref();
            vec![self.arg_1.check(
                "arg_1",
                &call.arg_1,
                (&ArgPrinter(&&call.arg_1)).debug_string(),
            )]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        pub accept_ref: FnData<'__rs, TraitMock<'__rs, 'a, 'b>, false, false>,
        pub return_mut_ref: FnData<'__rs, TraitMock<'__rs, 'a, 'b>, false, false>,
        pub return_mut_ref_with_base: FnData<'__rs, TraitMock<'__rs, 'a, 'b>, true, false>,
        pub foo_sum: FnData<'__rs, TraitMock<'__rs, 'a, 'b>, true, false>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitMockSetup<'__rs, 'a, 'b> {
        data: Arc<TraitMockData<'__rs, 'a, 'b>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitMockReceived<'__rs, 'a, 'b> {
        data: Arc<TraitMockData<'__rs, 'a, 'b>>,
    }
    #[derive(CloneForRSubstitute)]
    pub struct TraitMock<'__rs, 'a, 'b> {
        pub setup: TraitMockSetup<'__rs, 'a, 'b>,
        pub received: TraitMockReceived<'__rs, 'a, 'b>,
        pub data: Arc<TraitMockData<'__rs, 'a, 'b>>,
    }
    impl<'__rs, 'a, 'b> Trait<'a, 'b> for TraitMock<'__rs, 'a, 'b> {
        fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32 {
            let call: accept_ref_Call<'_, 'a, 'b> = accept_ref_Call {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                r: transmute_lifetime!(r),
            };
            return self.data.accept_ref.handle_returning(&self, call);
        }
        fn return_mut_ref(&self) -> &'a mut i32 {
            let call: return_mut_ref_Call<'_, 'a, 'b> = return_mut_ref_Call {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
            };
            return self.data.return_mut_ref.handle_returning(&self, call);
        }
        fn return_mut_ref_with_base(&self) -> &'a mut i32 {
            let call: return_mut_ref_with_base_Call<'_, 'a, 'b> = return_mut_ref_with_base_Call {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
            };
            return self.data.return_mut_ref_with_base.handle_base_returning(
                &self,
                call,
                Self::base_return_mut_ref_with_base,
            );
        }

        fn foo_sum(&mut self, arg_1: Foo) -> i32 {
            let call: foo_sum_Call<'_, 'a, 'b> = foo_sum_Call {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                arg_1: transmute_lifetime!(arg_1),
            };
            return self
                .data
                .foo_sum
                .handle_base_returning(&self, call, Self::base_foo_sum);
        }
    }
    impl<'__rs, 'a, 'b> TraitMock<'__rs, 'a, 'b> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                accept_ref: FnData::new("accept_ref"),
                return_mut_ref: FnData::new("return_mut_ref"),
                return_mut_ref_with_base: FnData::new("return_mut_ref_with_base"),
                foo_sum: FnData::new("foo_sum"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
        fn base_return_mut_ref_with_base(
            &self,
            call: return_mut_ref_with_base_Call<'__rs, 'a, 'b>,
        ) -> &'a mut i32 {
            let return_mut_ref_with_base_Call { .. } = call;
            unsafe { &mut *&raw mut BASE_MUT_REF }
        }

        fn base_foo_sum(&self, call: foo_sum_Call<'__rs, 'a, 'b>) -> i32 {
            let foo_sum_Call {
                arg_1: Foo { mut numbers },
                ..
            } = call;
            for number in numbers.iter_mut() {
                *number *= 2;
            }
            return numbers.into_iter().sum();
        }
    }
    impl<'__rs, 'a, 'b> TraitMockSetup<'__rs, 'a, 'b> {
        pub fn accept_ref<'__rsa: 'a + 'b>(
            &self,
            r: impl Into<Arg<'__rsa, &'a &'b &'a &'__rsa i32>>,
        ) -> FnTuner<
            '_,
            TraitMock<'__rs, 'a, 'b>,
            Self,
            (&'__rs &'a &'b &'a &'__rs i32,),
            &'b &'a &'b &'a i32,
            false,
            false,
        > {
            let accept_ref_args_checker: accept_ref_ArgsChecker<'_, 'a, 'b> =
                accept_ref_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    r: r.into(),
                };
            let fn_tuner: FnTuner<
                '_,
                TraitMock<'__rs, 'a, 'b>,
                Self,
                (&'__rs &'a &'b &'a &'__rs i32,),
                &'b &'a &'b &'a i32,
                false,
                false,
            > = self
                .data
                .accept_ref
                .add_config(accept_ref_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn return_mut_ref<'__rsa>(
            &self,
        ) -> FnTuner<'_, TraitMock<'__rs, 'a, 'b>, Self, (), &'a mut i32, false, false> {
            let return_mut_ref_args_checker: return_mut_ref_ArgsChecker<'_, 'a, 'b> =
                return_mut_ref_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                };
            let fn_tuner: FnTuner<
                '_,
                TraitMock<'__rs, 'a, 'b>,
                Self,
                (),
                &'a mut i32,
                false,
                false,
            > = self
                .data
                .return_mut_ref
                .add_config(return_mut_ref_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn return_mut_ref_with_base<'__rsa>(
            &self,
        ) -> FnTuner<'_, TraitMock<'__rs, 'a, 'b>, Self, (), &'a mut i32, true, false> {
            let return_mut_ref_with_base_args_checker: return_mut_ref_with_base_ArgsChecker<
                '_,
                'a,
                'b,
            > = return_mut_ref_with_base_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
            };
            let fn_tuner: FnTuner<
                '_,
                TraitMock<'__rs, 'a, 'b>,
                Self,
                (),
                &'a mut i32,
                true,
                false,
            > = self
                .data
                .return_mut_ref_with_base
                .add_config(return_mut_ref_with_base_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn foo_sum<'__rsa>(
            &self,
            arg_1: impl Into<Arg<'__rsa, Foo>>,
        ) -> FnTuner<'_, TraitMock<'__rs, 'a, 'b>, Self, (&'__rs Foo,), i32, true, false> {
            let foo_sum_args_checker: foo_sum_ArgsChecker<'_, 'a, 'b> = foo_sum_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                arg_1: arg_1.into(),
            };
            let fn_tuner: FnTuner<
                '_,
                TraitMock<'__rs, 'a, 'b>,
                Self,
                (&'__rs Foo,),
                i32,
                true,
                false,
            > = self.data.foo_sum.add_config(foo_sum_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'__rs, 'a, 'b> TraitMockReceived<'__rs, 'a, 'b> {
        pub fn accept_ref<'__rsa: 'a + 'b>(
            &self,
            r: impl Into<Arg<'__rsa, &'a &'b &'a &'__rsa i32>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rs &'a &'b &'a &'__rs i32,)> {
            let accept_ref_args_checker: accept_ref_ArgsChecker<'_, 'a, 'b> =
                accept_ref_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    r: r.into(),
                };
            self.data
                .accept_ref
                .verify_received(accept_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn return_mut_ref<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let return_mut_ref_args_checker: return_mut_ref_ArgsChecker<'_, 'a, 'b> =
                return_mut_ref_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                };
            self.data
                .return_mut_ref
                .verify_received(return_mut_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn return_mut_ref_with_base<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let return_mut_ref_with_base_args_checker: return_mut_ref_with_base_ArgsChecker<
                '_,
                'a,
                'b,
            > = return_mut_ref_with_base_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
            };
            self.data
                .return_mut_ref_with_base
                .verify_received(return_mut_ref_with_base_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn foo_sum(
            &self,
            arg_1: impl Into<Arg<'__rs, Foo>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rs Foo,)> {
            let foo_sum_args_checker: foo_sum_ArgsChecker<'_, 'a, 'b> = foo_sum_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                arg_1: arg_1.into(),
            };
            self.data
                .foo_sum
                .verify_received(foo_sum_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
