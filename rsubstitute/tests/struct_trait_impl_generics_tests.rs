use rsubstitute::*;

trait Trait<T> {
    fn work(&self, t: T) -> T;
}

#[mock]
struct Struct;

#[mock(base)]
impl Struct {}
impl<T: Clone> Trait<T> for Struct {
    fn work(&self, t: T) -> T {
        use __rsubstitute_generated_Struct_1_1::__rs_base___rsubstitute_generated_Struct_1_1_Trait; // TODO - add this usage
                                                                                                    // when generating base associated fns
        let call = __rsubstitute_generated_Struct_1_1::Trait_work_Call::<'_, T> {
            __rs_generics: ::core::marker::PhantomData,
            t: ::rsubstitute::transmute_lifetime!(t),
        };
        let fn_data: &::rsubstitute::for_generated::FnData<Struct, true, true, true> =
            ::rsubstitute::for_generated::ISharedMockData::get_shared_fn_data(
                &self.__rs_data,
                "Trait::work",
                ::rsubstitute::for_generated::IGenericsInfoProvider::get_generics_hash_key(&call),
            );
        fn_data.handle(self, call, Self::__rs_base_Trait_work)
    }
}
#[allow(private_interfaces)]
#[allow(unreachable_pub)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub mod __rsubstitute_generated_Struct_1_1 {
    use super::__rsubstitute_generated_StructMock::*;
    #[allow(unused_imports)]
    use super::*;
    pub struct Trait_work_Call<'__rsa, T: Clone> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (), Box<T>, T)>,
        pub(super) t: T,
    }
    impl<'__rsa, T: Clone> ::rsubstitute::for_generated::IGenericsInfoProvider
        for Trait_work_Call<'__rsa, T>
    {
        fn get_generic_parameter_infos(
            &self,
        ) -> Vec<::rsubstitute::for_generated::GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(
            &self,
            #[allow(unused_variables)] hasher: &mut ::rsubstitute::for_generated::GenericsHasher,
        ) {
        }
        fn hash_const_values(
            &self,
            #[allow(unused_variables)] hasher: &mut ::rsubstitute::for_generated::GenericsHasher,
        ) {
        }
    }
    impl<'__rsa, T: Clone> ::rsubstitute::for_generated::ICall for Trait_work_Call<'__rsa, T> {
        fn get_arg_infos(&self) -> Vec<::rsubstitute::for_generated::ArgInfo> {
            use ::rsubstitute::for_generated::arg_printing::*;
            vec![::rsubstitute::for_generated::ArgInfo::new(
                "t",
                &self.t,
                (&::rsubstitute::for_generated::ArgPrinter(::rsubstitute::transmute_lifetime!(
                    &self.t, &T
                )))
                    .debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.t,))) as *mut _ as *mut ()
        }
    }
    impl<'__rsa, T: Clone> ::core::clone::Clone for Trait_work_Call<'__rsa, T> {
        #[inline]
        fn clone(&self) -> Trait_work_Call<'__rsa, T> {
            Trait_work_Call::<'__rsa, T> {
                __rs_generics: ::core::clone::Clone::clone(&self.__rs_generics),
                t: ::core::clone::Clone::clone(&self.t),
            }
        }
    }
    struct Trait_work_ArgsChecker<'__rsa, T: Clone> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (), Box<T>, T)>,
        t: ::rsubstitute::for_generated::Arg<T>,
    }
    impl<'__rsa, T: Clone> ::rsubstitute::for_generated::IGenericsInfoProvider
        for Trait_work_ArgsChecker<'__rsa, T>
    {
        fn get_generic_parameter_infos(
            &self,
        ) -> Vec<::rsubstitute::for_generated::GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(
            &self,
            #[allow(unused_variables)] hasher: &mut ::rsubstitute::for_generated::GenericsHasher,
        ) {
        }
        fn hash_const_values(
            &self,
            #[allow(unused_variables)] hasher: &mut ::rsubstitute::for_generated::GenericsHasher,
        ) {
        }
    }
    impl<'__rsa, T: Clone> ::rsubstitute::for_generated::IArgsChecker
        for Trait_work_ArgsChecker<'__rsa, T>
    {
        fn check(
            &self,
            dyn_call: &::rsubstitute::for_generated::DynCall,
        ) -> Vec<::rsubstitute::for_generated::ArgCheckResult> {
            use ::rsubstitute::for_generated::arg_printing::*;
            #[allow(unused_variables)]
            let call: &Trait_work_Call<'__rsa, T> = dyn_call.downcast_ref();
            vec![
                ::rsubstitute::transmute_lifetime!(
                    &self.t,
                    &::rsubstitute::for_generated::Arg::<T>
                )
                .check(
                    "t",
                    ::rsubstitute::transmute_lifetime!(&call.t),
                    (&::rsubstitute::for_generated::ArgPrinter(
                        ::rsubstitute::transmute_lifetime!(&call.t, &T),
                    ))
                        .debug_string(),
                ),
            ]
        }
        fn fmt_args(&self) -> String {
            use ::rsubstitute::for_generated::arg_printing::*;
            format!(
                "{}",
                (&::rsubstitute::for_generated::ArgPrinter(::rsubstitute::transmute_lifetime!(
                    &&self.t,
                    &&::rsubstitute::for_generated::Arg::<T>
                )))
                    .debug_string()
            )
        }
    }
    pub trait __rs_base___rsubstitute_generated_Struct_1_1_Trait<T: Clone> {
        fn __generics() -> ::core::marker::PhantomData<(Box<T>,)> {
            ::core::marker::PhantomData
        }
        fn __rs_base_Trait_work<'__rs_ret>(
            __rsa_self: &'__rs_ret Struct,
            call: __rsubstitute_generated_Struct_1_1::Trait_work_Call<'_, T>,
        ) -> T;
    }
    impl<T: Clone> __rs_base___rsubstitute_generated_Struct_1_1_Trait<T> for Struct {
        #[doc(hidden)]
        fn __rs_base_Trait_work<'__rs_ret>(
            __rsa_self: &'__rs_ret Struct,
            call: __rsubstitute_generated_Struct_1_1::Trait_work_Call<'_, T>,
        ) -> T {
            let __rsubstitute_generated_Struct_1_1::Trait_work_Call::<'_, T> { t, .. } = call;
            let t: T = ::rsubstitute::transmute_lifetime!(t);
            { t }
        }
    }
    pub struct StructTraitSetup<'__rsa, T: Clone> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (), Box<T>)>,
        #[doc(hidden)]
        pub __rs_data: ::rsubstitute::for_generated::SharedMockData,
    }
    impl<'__rsa, T: Clone> StructTraitSetup<'__rsa, T> {
        pub fn work(
            &self,
            t: impl Into<::rsubstitute::for_generated::Arg<T>>,
        ) -> ::rsubstitute::for_generated::FnConfigurator<
            '_,
            Struct,
            Self,
            (&'__rsa T,),
            T,
            &Struct,
            true,
            true,
            true,
        > {
            let args_checker = Trait_work_ArgsChecker::<'__rsa, T> {
                __rs_generics: ::core::marker::PhantomData,
                t: ::rsubstitute::transmute_lifetime!(t.into()),
            };
            let fn_data: &::rsubstitute::for_generated::FnData<Struct, true, true, true> =
                ::rsubstitute::for_generated::ISharedMockData::get_shared_fn_data(
                    &self.__rs_data,
                    "Trait::work",
                    ::rsubstitute::for_generated::IGenericsInfoProvider::get_generics_hash_key(
                        &args_checker,
                    ),
                );
            let fn_configurator: ::rsubstitute::for_generated::FnConfigurator<
                '_,
                Struct,
                Self,
                (&'__rsa T,),
                T,
                &Struct,
                true,
                true,
                true,
            > = fn_data.add_config(args_checker, self);
            ::rsubstitute::transmute_lifetime!(fn_configurator)
        }
    }
    pub struct StructTraitReceived<'__rsa, T: Clone> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (), Box<T>)>,
        #[doc(hidden)]
        pub __rs_data: ::rsubstitute::for_generated::SharedMockData,
    }
    impl<'__rsa, T: Clone> ::core::clone::Clone for StructTraitReceived<'__rsa, T> {
        #[inline]
        fn clone(&self) -> StructTraitReceived<'__rsa, T> {
            StructTraitReceived::<'__rsa, T> {
                __rs_generics: ::core::clone::Clone::clone(&self.__rs_generics),
                __rs_data: ::core::clone::Clone::clone(&self.__rs_data),
            }
        }
    }
    impl<'__rsa, T: Clone> StructTraitReceived<'__rsa, T> {
        pub fn work(
            &self,
            t: impl Into<::rsubstitute::for_generated::Arg<T>>,
            times: ::rsubstitute::for_generated::Times,
        ) -> ::rsubstitute::for_generated::ArgRefsBinder<Self, (&'__rsa T,)> {
            let args_checker = Trait_work_ArgsChecker::<'__rsa, T> {
                __rs_generics: ::core::marker::PhantomData,
                t: ::rsubstitute::transmute_lifetime!(t.into()),
            };
            let fn_data: &::rsubstitute::for_generated::FnData<Struct, true, true, true> =
                ::rsubstitute::for_generated::ISharedMockData::get_shared_fn_data(
                    &self.__rs_data,
                    "Trait::work",
                    ::rsubstitute::for_generated::IGenericsInfoProvider::get_generics_hash_key(
                        &args_checker,
                    ),
                );
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn no_other_calls(&self) {
            rsubstitute::for_generated::IMockData::verify_received_nothing_else(&self.__rs_data)
        }
    }
    impl<'__rsa> StructSetup<'__rsa> {
        #[allow(non_snake_case)]
        pub fn as_Trait<T: Clone>(&self) -> StructTraitSetup<'__rsa, T> {
            StructTraitSetup::<'__rsa, T> {
                __rs_data: self.__rs_data.clone(),
                __rs_generics: ::core::marker::PhantomData,
            }
        }
    }
    impl<'__rsa> StructReceived<'__rsa> {
        #[allow(non_snake_case)]
        pub fn as_Trait<T: Clone>(&self) -> StructTraitReceived<'__rsa, T> {
            StructTraitReceived::<'__rsa, T> {
                __rs_data: self.__rs_data.clone(),
                __rs_generics: ::core::marker::PhantomData,
            }
        }
    }
}

mod tests {
    #[test]
    fn compile() {}
}
