trait Trait<T1> {
    fn f<T2>(&self) -> T1;
    fn g<T3>();
}

mod result {
    use Trait_mock::*;
    mod Trait_mock {
        use super::*;
        use rsubstitute_core::args::{
            ArgInfo, GenericParameterInfo, GenericsHasher, IGenericsInfoProvider,
        };
        use rsubstitute_core::fn_parameters::ICall;
        use rsubstitute_core::infrastructure::{
            get_static_fn_data, FnConfigurator, FnData, ISharedMockData, SharedMockData,
        };
        use std::marker::PhantomData;

        pub trait Trait<T1> {
            fn f<T2>(&self) -> T1;
            fn g<T3>();
        }

        pub struct TraitMock<T1> {
            #[doc(hidden)]
            pub data: SharedMockData<TraitMock<T1>>,
        }

        #[derive(Clone)]
        struct fCall;
        impl IGenericsInfoProvider for fCall {
            fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
                vec![]
            }

            fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        }
        impl ICall for fCall {
            fn get_arg_infos(&self) -> Vec<ArgInfo> {
                vec![]
            }

            fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
                core::ptr::null_mut()
            }
        }

        impl<T1> Trait<T1> for TraitMock<T1> {
            fn f<T2>(&self) -> T1 {
                let fn_data: &FnData<TraitMock<T1>, true, true, true> =
                    self.data.get_shared_fn_data("f");
                fn_data.handle(self, fCall, Self::__rs_base_f::<T2>)
            }

            fn g<T3>() {
                let fn_data: &FnData<TraitMock<T1>, true, true, false> = get_static_fn_data("g");
                fn_data.handle((), fCall, Self::__rs_base_g::<T3>);
                todo!()
            }
        }

        impl<T1> TraitMock<T1> {
            pub fn new() -> Self {
                Self {
                    data: Default::default(),
                }
            }

            pub fn setup(&mut self) -> TraitSetup<T1> {
                TraitSetup {
                    data: self.data.clone(),
                }
            }
            
            pub fn static_setup() -> TraitStaticSetup<T1> {
                TraitStaticSetup {
                    _generics: PhantomData,
                }
            }

            fn __rs_base_f<T2>(&self, _: fCall) -> T1 {
                todo!()
            }

            fn __rs_base_g<T3>(_: (), _: fCall) {}
        }

        pub struct TraitSetup<T1> {
            #[doc(hidden)]
            pub data: SharedMockData<TraitMock<T1>>,
        }
        pub struct TraitStaticSetup<T1> {
            _generics: PhantomData<(T1,)>,
        }

        impl<T1> TraitSetup<T1> {
            pub fn f<'__rsa, T2>(
                &mut self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                TraitMock<T1>,
                TraitSetup<T1>,
                (),
                T1,
                TraitMock<T1>,
                true,
                true,
                false,
            > {
                let fn_data: &FnData<TraitMock<T1>, true, true, false> =
                    self.data.get_shared_fn_data("f");
                    // fn_data.add_config()
                todo!()
            }
        }

        impl<T1> TraitStaticSetup<T1> {
            pub fn g<'__rsa, T3>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                TraitMock<T1>,
                TraitStaticSetup<T1>,
                (),
                (),
                TraitMock<T1>,
                false,
                true,
                false,
            > {
                let fn_data: &FnData<TraitMock<T1>, false, true, false> = get_static_fn_data("g");
                todo!()
            }
        }
    }

    use kavo::*;
    mod kavo {
        pub trait IStaticSetup {
            type Setup;
            fn static_setup() -> Self::Setup;
        }
    }

    fn usage() {
        let mut trait_mock = TraitMock::new();
        trait_mock.setup().f::<[u8; 1]>(1).returns("amogus");
        TraitMock::<[u8; 11]>::static_setup()
            .g::<Vec<f32>>(443)
            .call_base();
    }
}
