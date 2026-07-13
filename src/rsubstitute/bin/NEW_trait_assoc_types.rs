#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]
#![feature(associated_type_defaults)]

#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::fmt::Debug;

pub use __rsubstitute_generated_TraitMock::{Trait, TraitMock};
#[allow(non_camel_case_types)]
mod __rsubstitute_generated_TraitMock {
    #[allow(unused_imports)]
    use super::*;
    use rsubstitute::for_generated::*;
    pub trait Trait {
        const CONST: usize = 43;

        type InputType<TAmogus: Clone>: Clone + Debug
            = i32
        where
            Self: Clone;

        type OutputType<TT>: Clone + Sized + Default
            = u8
        where
            Self: Sized,
            TT: Clone;

        fn get_const(&self) -> usize {
            Self::CONST
        }

        fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
        where
            Self: Clone + Sized,
            TT: ToString;
    }
    pub struct get_const_Call<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > {
        generics: PhantomData<(Trait_InputType, Trait_OutputType)>,
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > IGenericsInfoProvider for get_const_Call<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > ICall for get_const_Call<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > ::core::clone::Clone for get_const_Call<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        #[inline]
        fn clone(&self) -> get_const_Call<Trait_InputType, Trait_OutputType, Trait_CONST> {
            get_const_Call::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                generics: ::core::clone::Clone::clone(&self.generics),
            }
        }
    }
    struct get_const_ArgsChecker<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > {
        generics: PhantomData<(Trait_InputType, Trait_OutputType)>,
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > IGenericsInfoProvider
        for get_const_ArgsChecker<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > IArgsChecker for get_const_ArgsChecker<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &get_const_Call<Trait_InputType, Trait_OutputType, Trait_CONST> =
                dyn_call.downcast_ref();
            vec![]
        }
        fn fmt_args(&self) -> String {
            format!("")
        }
    }
    pub struct get_my_type_Call<
        TT: Clone,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    >
    where
        Self: Clone + Sized,
        TT: ToString,
    {
        generics: PhantomData<(TT, Trait_InputType, Trait_OutputType, Trait_InputType)>,
        input: Trait_InputType,
    }
    impl<
        TT: Clone,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > IGenericsInfoProvider for get_my_type_Call<TT, Trait_InputType, Trait_OutputType, Trait_CONST>
    where
        Self: Clone + Sized,
        TT: ToString,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("TT", core::any::type_name::<TT>())]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            [tid::<TT>()].hash(hasher);
        }
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<
        TT: Clone,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > ICall for get_my_type_Call<TT, Trait_InputType, Trait_OutputType, Trait_CONST>
    where
        Self: Clone + Sized,
        TT: ToString,
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "input",
                &self.input,
                (&ArgPrinter(transmute_lifetime!(&self.input, &Trait_InputType))).debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.input,))) as *mut _ as *mut ()
        }
    }
    struct get_my_type_ArgsChecker<
        TT: Clone,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    >
    where
        Self: Clone + Sized,
        TT: ToString,
    {
        generics: PhantomData<(TT, Trait_InputType, Trait_OutputType, Trait_InputType)>,
        input: Arg<Trait_InputType>,
    }
    impl<
        TT: Clone,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > IGenericsInfoProvider
        for get_my_type_ArgsChecker<TT, Trait_InputType, Trait_OutputType, Trait_CONST>
    where
        Self: Clone + Sized,
        TT: ToString,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("TT", core::any::type_name::<TT>())]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            [tid::<TT>()].hash(hasher);
        }
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<
        TT: Clone,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > IArgsChecker for get_my_type_ArgsChecker<TT, Trait_InputType, Trait_OutputType, Trait_CONST>
    where
        Self: Clone + Sized,
        TT: ToString,
    {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &get_my_type_Call<
                TT,
                Trait_InputType,
                Trait_OutputType,
                Trait_CONST,
            > = dyn_call.downcast_ref();
            vec![
                transmute_lifetime!(&self.input, &Arg<Trait_InputType>).check(
                    "input",
                    transmute_lifetime!(&call.input),
                    (&ArgPrinter(transmute_lifetime!(&call.input, &Trait_InputType)))
                        .debug_string(),
                ),
            ]
        }
        fn fmt_args(&self) -> String {
            format!(
                "{}",
                (&ArgPrinter(transmute_lifetime!(&&self.input, &&Arg<Trait_InputType>)))
                    .debug_string()
            )
        }
    }
    pub struct TraitMock<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > {
        generics: PhantomData<(Trait_InputType, Trait_OutputType)>,
        pub data: ::rsubstitute::for_generated::SharedMockData,
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > Trait for TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        const CONST: usize = Trait_CONST;
        type InputType<TAmogus: Clone>
            = Trait_InputType
        where
            Self: Clone;
        type OutputType<TT>
            = Trait_OutputType
        where
            Self: Sized,
            TT: Clone;
        fn get_const(&self) -> usize {
            let call = get_const_Call::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                generics: ::core::marker::PhantomData,
            };
            let fn_data: &FnData<
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                true,
                true,
                false,
            > = self
                .data
                .get_shared_fn_data("get_const", call.get_generics_hash_key());
            fn_data.handle(
                &TraitMock::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                    generics: ::core::marker::PhantomData,
                    data: self.data.clone(),
                },
                call,
                Self::__rs_base_get_const,
            )
        }
        fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            let call = get_my_type_Call::<TT, Trait_InputType, Trait_OutputType, Trait_CONST> {
                generics: ::core::marker::PhantomData,
                input: transmute_lifetime!(input),
            };
            let fn_data: &FnData<
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                true,
                false,
                false,
            > = self
                .data
                .get_shared_fn_data("get_my_type", call.get_generics_hash_key());
            fn_data.handle(
                &TraitMock::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                    generics: ::core::marker::PhantomData,
                    data: self.data.clone(),
                },
                call,
            )
        }
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        pub fn new() -> Self {
            Self {
                generics: ::core::marker::PhantomData,
                data: ::core::default::Default::default(),
            }
        }
        pub fn setup(&mut self) -> TraitSetup<Trait_InputType, Trait_OutputType, Trait_CONST> {
            TraitSetup::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                generics: ::core::marker::PhantomData,
                data: self.data.clone(),
            }
        }
        pub fn received(
            &mut self,
        ) -> TraitReceived<Trait_InputType, Trait_OutputType, Trait_CONST> {
            TraitReceived::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                generics: ::core::marker::PhantomData,
                data: self.data.clone(),
            }
        }
        fn __rs_base_get_const(
            __rsa_self: &TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
            call: get_const_Call<Trait_InputType, Trait_OutputType, Trait_CONST>,
        ) -> usize {
            let get_const_Call::<Trait_InputType, Trait_OutputType, Trait_CONST> { .. } = call;
            let (): () = transmute_lifetime!(());
            { Self::CONST }
        }
    }
    pub struct TraitSetup<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > {
        generics: PhantomData<(Trait_InputType, Trait_OutputType)>,
        data: ::rsubstitute::for_generated::SharedMockData,
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > TraitSetup<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        pub fn get_const<'__rsa>(
            &self,
        ) -> FnConfigurator<
            '_,
            TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
            Self,
            (),
            usize,
            TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
            true,
            true,
            false,
        > {
            let args_checker =
                get_const_ArgsChecker::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                    generics: ::core::marker::PhantomData,
                };
            let fn_data: &FnData<
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                true,
                true,
                false,
            > = self
                .data
                .get_shared_fn_data("get_const", args_checker.get_generics_hash_key());
            let fn_configurator: FnConfigurator<
                '_,
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                Self,
                (),
                usize,
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                true,
                true,
                false,
            > = fn_data.add_config(args_checker, self);
            transmute_lifetime!(fn_configurator)
        }
        pub fn get_my_type<'__rsa, TT: Clone>(
            &self,
            input: impl Into<Arg<Trait_InputType>>,
        ) -> FnConfigurator<
            '_,
            TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
            Self,
            (&'__rsa Trait_InputType,),
            Trait_OutputType,
            TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
            true,
            false,
            false,
        >
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            let args_checker =
                get_my_type_ArgsChecker::<TT, Trait_InputType, Trait_OutputType, Trait_CONST> {
                    generics: ::core::marker::PhantomData,
                    input: transmute_lifetime!(input.into()),
                };
            let fn_data: &FnData<
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                true,
                false,
                false,
            > = self
                .data
                .get_shared_fn_data("get_my_type", args_checker.get_generics_hash_key());
            let fn_configurator: FnConfigurator<
                '_,
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                Self,
                (&'__rsa Trait_InputType,),
                Trait_OutputType,
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                true,
                false,
                false,
            > = fn_data.add_config(args_checker, self);
            transmute_lifetime!(fn_configurator)
        }
    }
    pub struct TraitReceived<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > {
        generics: PhantomData<(Trait_InputType, Trait_OutputType)>,
        data: ::rsubstitute::for_generated::SharedMockData,
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > ::core::clone::Clone for TraitReceived<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        #[inline]
        fn clone(&self) -> TraitReceived<Trait_InputType, Trait_OutputType, Trait_CONST> {
            TraitReceived::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                generics: ::core::clone::Clone::clone(&self.generics),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
    }
    impl<
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        const Trait_CONST: usize,
    > TraitReceived<Trait_InputType, Trait_OutputType, Trait_CONST>
    {
        pub fn get_const<'__rsa>(
            &self,
            times: Times,
        ) -> ::rsubstitute::for_generated::ArgRefsBinder<Self, ()> {
            let args_checker =
                get_const_ArgsChecker::<Trait_InputType, Trait_OutputType, Trait_CONST> {
                    generics: ::core::marker::PhantomData,
                };
            let fn_data: &FnData<
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                true,
                true,
                false,
            > = self
                .data
                .get_shared_fn_data("get_const", args_checker.get_generics_hash_key());
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn get_my_type<'__rsa, TT: Clone>(
            &self,
            input: impl Into<Arg<Trait_InputType>>,
            times: Times,
        ) -> ::rsubstitute::for_generated::ArgRefsBinder<Self, (&'__rsa Trait_InputType,)>
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            let args_checker =
                get_my_type_ArgsChecker::<TT, Trait_InputType, Trait_OutputType, Trait_CONST> {
                    generics: ::core::marker::PhantomData,
                    input: transmute_lifetime!(input.into()),
                };
            let fn_data: &FnData<
                TraitMock<Trait_InputType, Trait_OutputType, Trait_CONST>,
                true,
                false,
                false,
            > = self
                .data
                .get_shared_fn_data("get_my_type", args_checker.get_generics_hash_key());
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn no_other_calls(&self) {
            self.data
                .verify_received_nothing_else(["get_const", "get_my_type"])
        }
    }
}

fn main() {
    println!("Done");
}
