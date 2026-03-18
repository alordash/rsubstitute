#![feature(associated_type_defaults)]

use rsubstitute::prelude::*;

// #[mock]
// trait Trait {
//     const MY_CONST: usize = 43;
//
//     type MyType<TT>: Clone + Sized
//         = u8
//     where
//         Self: Sized,
//         TT: Clone;
//
//     fn get_const(&self) -> usize {
//         Self::MY_CONST
//     }
//
//     fn get_my_type<TT: Clone>(&self) -> Self::MyType<TT>
//     where
//         Self: Sized;
// }

// mocked! {
//     struct Struct;
//
//     impl Struct {
//         pub fn new() -> Self {
//             Self
//         }
//     }
//
//     impl Trait for Struct {
//         const MY_CONST: usize = 643;
//         type MyType<TT>
//         = Vec<TT>
//     where
//         Self: Sized,
//         TT: Clone;
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[test]
    fn compile() {
        // let mock = TraitMock::<3, i32>::new();
    }
}

trait Trait {
    const MY_CONST: usize = 43;

    type MyType<TT>: Clone + Sized
        = u8
    where
        Self: Sized,
        TT: Clone;

    fn get_const(&self) -> usize {
        Self::MY_CONST
    }

    fn get_my_type<TT: Clone>(&self) -> Self::MyType<TT>
    where
        Self: Sized;
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
    pub struct get_const_Call<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_Trait_MyType: PhantomData<Trait_MyType>,
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> IGenericsInfoProvider
        for get_const_Call<'__rs, Trait_MY_CONST, Trait_MyType>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter)]
    pub struct get_const_ArgsChecker<
        '__rs,
        const Trait_MY_CONST: usize,
        Trait_MyType: Clone + Sized,
    > {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_Trait_MyType: PhantomData<Trait_MyType>,
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> IArgsChecker
        for get_const_ArgsChecker<'__rs, Trait_MY_CONST, Trait_MyType>
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &get_const_Call<'__rs, Trait_MY_CONST, Trait_MyType> =
                dyn_call.downcast_ref();
            vec![]
        }
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> IGenericsInfoProvider
        for get_const_ArgsChecker<'__rs, Trait_MY_CONST, Trait_MyType>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider)]
    pub struct get_my_type_Call<
        '__rs,
        const Trait_MY_CONST: usize,
        Trait_MyType: Clone + Sized,
        TT: Clone,
    >
    where
        Self: Sized,
    {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_Trait_MyType: PhantomData<Trait_MyType>,
        _phantom_TT: PhantomData<TT>,
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized, TT: Clone>
        IGenericsInfoProvider for get_my_type_Call<'__rs, Trait_MY_CONST, Trait_MyType, TT>
    where
        Self: Sized,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("TT", core::any::type_name::<TT>())]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<TT>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter)]
    pub struct get_my_type_ArgsChecker<
        '__rs,
        const Trait_MY_CONST: usize,
        Trait_MyType: Clone + Sized,
        TT: Clone,
    >
    where
        Self: Sized,
    {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_Trait_MyType: PhantomData<Trait_MyType>,
        _phantom_TT: PhantomData<TT>,
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized, TT: Clone> IArgsChecker
        for get_my_type_ArgsChecker<'__rs, Trait_MY_CONST, Trait_MyType, TT>
    where
        Self: Sized,
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &get_my_type_Call<'__rs, Trait_MY_CONST, Trait_MyType, TT> =
                dyn_call.downcast_ref();
            vec![]
        }
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized, TT: Clone>
        IGenericsInfoProvider for get_my_type_ArgsChecker<'__rs, Trait_MY_CONST, Trait_MyType, TT>
    where
        Self: Sized,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("TT", core::any::type_name::<TT>())]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<TT>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_Trait_MyType: PhantomData<Trait_MyType>,
        pub get_const:
            FnData<'static, TraitMock<'__rs, Trait_MY_CONST, Trait_MyType>, false, false>,
        pub get_my_type:
            FnData<'static, TraitMock<'__rs, Trait_MY_CONST, Trait_MyType>, false, false>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitMockSetup<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> {
        data: Arc<TraitMockData<'__rs, Trait_MY_CONST, Trait_MyType>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitMockReceived<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> {
        data: Arc<TraitMockData<'__rs, Trait_MY_CONST, Trait_MyType>>,
    }
    #[derive(CloneForRSubstitute)]
    pub struct TraitMock<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> {
        pub setup: TraitMockSetup<'__rs, Trait_MY_CONST, Trait_MyType>,
        pub received: TraitMockReceived<'__rs, Trait_MY_CONST, Trait_MyType>,
        pub data: Arc<TraitMockData<'__rs, Trait_MY_CONST, Trait_MyType>>,
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized> Trait
        for TraitMock<'__rs, Trait_MY_CONST, Trait_MyType>
    {
        const MY_CONST: usize = Trait_MY_CONST; // TODO -replaced manually
        type MyType<TT>
            = Trait_MyType
        where
            Self: Sized,
            TT: Clone;

        fn get_const(&self) -> usize {
            let call: get_const_Call<'_, Trait_MY_CONST, Trait_MyType> = get_const_Call {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_Trait_MyType: PhantomData,
            };
            return self.data.get_const.handle_returning(&self, call);
        }

        fn get_my_type<TT: Clone>(&self) -> Self::MyType<TT>
        where
            Self: Sized,
        {
            let call: get_my_type_Call<'_, Trait_MY_CONST, Trait_MyType, TT> = get_my_type_Call {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_Trait_MyType: PhantomData,
                _phantom_TT: PhantomData,
            };
            return self.data.get_my_type.handle_returning(&self, call);
        }
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized>
        TraitMock<'__rs, Trait_MY_CONST, Trait_MyType>
    {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_Trait_MyType: PhantomData,
                get_const: FnData::new("get_const"),
                get_my_type: FnData::new("get_my_type"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized>
        TraitMockSetup<'__rs, Trait_MY_CONST, Trait_MyType>
    {
        pub fn get_const<'__rsa>(
            &self,
        ) -> FnTuner<
            '_,
            TraitMock<'__rs, Trait_MY_CONST, Trait_MyType>,
            Self,
            (),
            usize,
            false,
            false,
        > {
            let get_const_args_checker: get_const_ArgsChecker<'_, Trait_MY_CONST, Trait_MyType> =
                get_const_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom___rs: PhantomData,
                    _phantom_Trait_MyType: PhantomData,
                };
            let fn_tuner: FnTuner<
                '_,
                TraitMock<'__rs, Trait_MY_CONST, Trait_MyType>,
                Self,
                (),
                usize,
                false,
                false,
            > = self.data.get_const.add_config(get_const_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn get_my_type<'__rsa, TT: Clone>(
            &self,
        ) -> FnTuner<
            '_,
            TraitMock<'__rs, Trait_MY_CONST, Trait_MyType>,
            Self,
            (),
            Trait_MyType,   // TODO - replaced manually
            false,
            false,
        >
        where
            Self: Sized,
        {
            let get_my_type_args_checker: get_my_type_ArgsChecker<
                '_,
                Trait_MY_CONST,
                Trait_MyType,
                TT,
            > = get_my_type_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_Trait_MyType: PhantomData,
                _phantom_TT: PhantomData,
            };
            let fn_tuner: FnTuner<
                '_,
                TraitMock<'__rs, Trait_MY_CONST, Trait_MyType>,
                Self,
                (),
                Trait_MyType,
                false,
                false,
            > = self
                .data
                .get_my_type
                .add_config(get_my_type_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'__rs, const Trait_MY_CONST: usize, Trait_MyType: Clone + Sized>
        TraitMockReceived<'__rs, Trait_MY_CONST, Trait_MyType>
    {
        pub fn get_const<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let get_const_args_checker: get_const_ArgsChecker<'_, Trait_MY_CONST, Trait_MyType> =
                get_const_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom___rs: PhantomData,
                    _phantom_Trait_MyType: PhantomData,
                };
            self.data
                .get_const
                .verify_received(get_const_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn get_my_type<'__rsa, TT: Clone>(&self, times: Times) -> FnVerifier<Self, ()>
        where
            Self: Sized,
        {
            let get_my_type_args_checker: get_my_type_ArgsChecker<
                '_,
                Trait_MY_CONST,
                Trait_MyType,
                TT,
            > = get_my_type_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_Trait_MyType: PhantomData,
                _phantom_TT: PhantomData,
            };
            self.data
                .get_my_type
                .verify_received(get_my_type_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
