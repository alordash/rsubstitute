use rsubstitute::prelude::*;

// #[mock]
// trait Trait {
//     fn anonymous_first<'a>(&self, i: &&'a &&'a &i32);
//
//     fn a_first<'a>(&self, i: &'a &&'a &&'a i32);
// }

#[cfg(test)]
mod tests {
    #[test]
    fn compiles() {}
}

trait Trait {
    fn anonymous_first<'a>(&self, i: &&'a &&'a &i32);

    fn a_first<'a>(&self, i: &'a &&'a &&'a i32);
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
    pub struct Trait_anonymous_first_Call<'a> {
        _phantom_i: PhantomData<*const &'a *const &'a *const i32>,
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        i: *const *const *const *const *const i32,
    }
    impl<'a> IArgsInfosProvider for Trait_anonymous_first_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "i",
                &self.i,
                (&ArgPrinter::<&&&&&i32>(transmute_lifetime!(&self.i))).debug_string(),
            )]
        }
    }
    impl<'a> IArgsTupleProvider for Trait_anonymous_first_Call<'a> {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.i,))) as *mut _ as *mut ()
        }
    }
    impl<'a> IGenericsInfoProvider for Trait_anonymous_first_Call<'a> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct Trait_anonymous_first_ArgsChecker<'a> {
        _phantom_i: PhantomData<*const &'a *const &'a *const i32>,
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        i: Arg<*const *const *const *const *const i32>,
    }
    impl<'a> IArgsChecker for Trait_anonymous_first_ArgsChecker<'a> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_anonymous_first_Call<'a> = dyn_call.downcast_ref();
            vec![self.i.check_ref(
                "i",
                transmute_lifetime!(&call.i),
                (&ArgPrinter::<&&&&&i32>(transmute_lifetime!(&call.i))).debug_string(),
            )]
        }
    }
    impl<'a> IArgsFormatter for Trait_anonymous_first_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            format!("{}", (&ArgPrinter(&self.i)).debug_string())
        }
    }
    impl<'a> IGenericsInfoProvider for Trait_anonymous_first_ArgsChecker<'a> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct Trait_a_first_Call<'a> {
        _phantom_i: PhantomData<&'a *const &'a *const &'a i32>,
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        i: *const *const *const *const *const i32,
    }
    impl<'a> IArgsInfosProvider for Trait_a_first_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "i",
                &self.i,
                (&ArgPrinter::<&&&&&i32>(transmute_lifetime!(&self.i))).debug_string(),
            )]
        }
    }
    impl<'a> IArgsTupleProvider for Trait_a_first_Call<'a> {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.i,))) as *mut _ as *mut ()
        }
    }
    impl<'a> IGenericsInfoProvider for Trait_a_first_Call<'a> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct Trait_a_first_ArgsChecker<'a> {
        _phantom_i: PhantomData<&'a *const &'a *const &'a i32>,
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        i: Arg<*const *const *const *const *const i32>,
    }
    impl<'a> IArgsChecker for Trait_a_first_ArgsChecker<'a> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_a_first_Call<'a> = dyn_call.downcast_ref();
            vec![self.i.check_ref(
                "i",
                transmute_lifetime!(&call.i),
                (&ArgPrinter::<&&&&&i32>(transmute_lifetime!(&call.i))).debug_string(),
            )]
        }
    }
    impl<'a> IArgsFormatter for Trait_a_first_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            format!("{}", (&ArgPrinter(&self.i)).debug_string())
        }
    }
    impl<'a> IGenericsInfoProvider for Trait_a_first_ArgsChecker<'a> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData {
        pub Trait_anonymous_first: FnData<'static, TraitMock, false, false>,
        pub Trait_a_first: FnData<'static, TraitMock, false, false>,
    }
    #[doc(hidden)]
    pub struct TraitMockSetup {
        data: Arc<TraitMockData>,
    }
    impl Clone for TraitMockSetup {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct TraitMockReceived {
        data: Arc<TraitMockData>,
    }
    impl Clone for TraitMockReceived {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    pub struct TraitMock {
        pub setup: TraitMockSetup,
        pub received: TraitMockReceived,
        pub data: Arc<TraitMockData>,
    }
    impl AsRef<TraitMock> for TraitMock {
        fn as_ref(&self) -> &TraitMock {
            self
        }
    }
    impl Clone for TraitMock {
        fn clone(&self) -> Self {
            Self {
                setup: (&self.setup).clone(),
                received: (&self.received).clone(),
                data: (&self.data).clone(),
            }
        }
    }
    impl Trait for TraitMock {
        fn anonymous_first<'a>(&self, i: &&'a &&'a &i32) {
            let call: Trait_anonymous_first_Call<'_> = Trait_anonymous_first_Call {
                _phantom_i: PhantomData,
                _phantom_GenericParam_a: PhantomData,
                i: transmute_lifetime!(i),
            };
            self.data.clone().Trait_anonymous_first.handle(self, call);
        }
        fn a_first<'a>(&self, i: &'a &&'a &&'a i32) {
            let call: Trait_a_first_Call<'_> = Trait_a_first_Call {
                _phantom_i: PhantomData,
                _phantom_GenericParam_a: PhantomData,
                i: transmute_lifetime!(i),
            };
            self.data.clone().Trait_a_first.handle(self, call);
        }
    }
    impl TraitMock {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                Trait_anonymous_first: FnData::new("Trait::anonymous_first"),
                Trait_a_first: FnData::new("Trait::a_first"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl TraitMockSetup {
        pub fn anonymous_first<'__rsa: 'a, 'a: '__rsa>(
            &self,
            i: impl Into<Arg<&'__rsa &'a &'__rsa &'a &'__rsa i32>>,
        ) -> FnTuner<'_, TraitMock, Self, (&&&'a &&'a &i32,), (), &Self, false, false> {
            let Trait_anonymous_first_args_checker: Trait_anonymous_first_ArgsChecker<'a> =
                Trait_anonymous_first_ArgsChecker {
                    _phantom_i: PhantomData,
                    _phantom_GenericParam_a: PhantomData,
                    i: transmute_lifetime!(i.into()),
                };
            let fn_tuner: FnTuner<
                '_,
                TraitMock,
                Self,
                (&&&'a &&'a &i32,),
                (),
                &Self,
                false,
                false,
            > = self
                .data
                .Trait_anonymous_first
                .add_config(Trait_anonymous_first_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn a_first<'__rsa: 'a, 'a: '__rsa>(
            &self,
            i: impl Into<Arg<&'a &'__rsa &'a &'__rsa &'a i32>>,
        ) -> FnTuner<'_, TraitMock, Self, (&&'a &&'a &&'a i32,), (), &Self, false, false> {
            let Trait_a_first_args_checker: Trait_a_first_ArgsChecker<'a> =
                Trait_a_first_ArgsChecker {
                    _phantom_i: PhantomData,
                    _phantom_GenericParam_a: PhantomData,
                    i: transmute_lifetime!(i.into()),
                };
            let fn_tuner: FnTuner<
                '_,
                TraitMock,
                Self,
                (&&'a &&'a &&'a i32,),
                (),
                &Self,
                false,
                false,
            > = self
                .data
                .Trait_a_first
                .add_config(Trait_a_first_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl TraitMockReceived {
        pub fn anonymous_first<'__rsa: 'a, 'a: '__rsa>(
            &self,
            i: impl Into<Arg<&'__rsa &'a &'__rsa &'a &'__rsa i32>>,
            times: Times,
        ) -> FnVerifier<Self, (&&&'a &&'a &i32,)> {
            let Trait_anonymous_first_args_checker: Trait_anonymous_first_ArgsChecker<'a> =
                Trait_anonymous_first_ArgsChecker {
                    _phantom_i: PhantomData,
                    _phantom_GenericParam_a: PhantomData,
                    i: transmute_lifetime!(i.into()),
                };
            self.data
                .Trait_anonymous_first
                .verify_received(Trait_anonymous_first_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn a_first<'__rsa: 'a, 'a: '__rsa>(
            &self,
            i: impl Into<Arg<&'a &'__rsa &'a &'__rsa &'a i32>>,
            times: Times,
        ) -> FnVerifier<Self, (&&'a &&'a &&'a i32,)> {
            let Trait_a_first_args_checker: Trait_a_first_ArgsChecker<'a> =
                Trait_a_first_ArgsChecker {
                    _phantom_i: PhantomData,
                    _phantom_GenericParam_a: PhantomData,
                    i: transmute_lifetime!(i.into()),
                };
            self.data
                .Trait_a_first
                .verify_received(Trait_a_first_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
