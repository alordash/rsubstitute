#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::args::Arg;
use rsubstitute_core::Times;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

use f::f;
#[allow(non_camel_case_types)]
pub mod f {
    #[allow(unused_imports)]
    use super::*;
    use rsubstitute::for_generated::*;
    pub fn f<'a, T: Clone>(v: &'a T) {
        let fn_data: &FnData<fMock<'a, T>, false, true, false> = get_static_fn_data("f");
        fn_data.handle(
            &fMock::<'a, T> {
                generics: PhantomData,
            },
            f_Call::<'a, T> {
                generics: PhantomData,
                v: transmute_lifetime!(v),
            },
            __rs_base_f::<T>,
        )
    }

    fn base_f<'a, T: Clone>(_: &fMock<'a, T>, call: f_Call<'a, T>) {
        #[allow(non_shorthand_field_patterns)]
        #[allow(unused_variables)]
        let f_Call::<'_, T> { v: v, .. } = call;
        let v: &'a T = transmute_lifetime!(v);
    }

    fn __rs_base_f<'a, T: Clone>(_: &fMock<'a, T>, call: f_Call<'a, T>) {
        let f_Call::<'a, T> { v: v, .. } = call;
        let (v,): (&'a T,) = transmute_lifetime!((v,));
        {}
    }
    pub fn setup<'__rsa, 'a, T: Clone>(
        v: impl Into<Arg<&'a T>>,
    ) -> FnConfigurator<
        '__rsa,
        fMock<'a, T>,
        fStaticSetup<'a, T>,
        (&'__rsa &'a T,),
        (),
        fMock<'a, T>,
        false,
        true,
        false,
    > {
        let fn_data: &FnData<fMock<'a, T>, false, true, false> = get_static_fn_data("f");
        fn_data.reset();
        fStaticSetup::<'a, T> {
            generics: PhantomData,
        }
        .setup(v)
    }
    pub fn received<'__rsa, 'a, T: Clone>(
        v: impl Into<Arg<&'a T>>,
        times: Times,
    ) -> fStaticReceived<'a, T> {
        fStaticReceived::<'a, T> {
            generics: PhantomData,
        }
        .received(v, times)
    }
    pub struct f_Call<'a, T: Clone> {
    // TODO - in generics there should be generic argument for each input and output
        generics: PhantomData<(&'a T, T)>,
        v: *const T,
    }
    impl<'a, T: Clone> IGenericsInfoProvider for f_Call<'a, T> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("T", core::any::type_name::<T>())]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            [tid::<T>()];
        }
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<'a, T: Clone> ICall for f_Call<'a, T> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "v",
                &self.v,
                (&ArgPrinter(transmute_lifetime!(&self.v, &&'a T))).debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.v,))) as *mut _ as *mut ()
        }
    }
    impl<'a, T: Clone> ::core::clone::Clone for f_Call<'a, T> {
        #[inline]
        fn clone(&self) -> f_Call<'a, T> {
            f_Call::<'a, T> {
                generics: (&self.generics).clone(),
                v: (&self.v).clone(),
            }
        }
    }
    struct f_ArgsChecker<'a, T: Clone> {
        generics: PhantomData<(T,)>,
        v: Arg<*const T>,
    }
    impl<'a, T: Clone> IGenericsInfoProvider for f_ArgsChecker<'a, T> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("T", core::any::type_name::<T>())]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            [tid::<T>()];
        }
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<'a, T: Clone> IArgsChecker for f_ArgsChecker<'a, T> {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &f_Call<'a, T> = dyn_call.downcast_ref();
            vec![transmute_lifetime!(&self.v, &Arg<&'a T>).check_ref(
                "v",
                transmute_lifetime!(&call.v),
                (&ArgPrinter(transmute_lifetime!(&call.v, &&'a T))).debug_string(),
            )]
        }
        fn fmt_args(&self) -> String {
            format!(
                "{}",
                (&ArgPrinter(transmute_lifetime!(&&self.v, &&Arg<&'a T>))).debug_string()
            )
        }
    }
    pub struct fMock<'a, T: Clone> {
        generics: PhantomData<(T,)>,
    }
    pub struct fStaticSetup<'a, T: Clone> {
        generics: PhantomData<(T,)>,
    }
    impl<'a, T: Clone> fStaticSetup<'a, T> {
        pub fn setup<'__rsa>(
            &self,
            v: impl Into<Arg<&'a T>>,
        ) -> FnConfigurator<
            '_,
            fMock<'a, T>,
            Self,
            (&'__rsa &'a T,),
            (),
            fMock<'a, T>,
            false,
            true,
            false,
        > {
            let fn_data: &FnData<fMock<'a, T>, false, true, false> = get_static_fn_data("f");
            let args_checker = f_ArgsChecker::<'a, T> {
                generics: PhantomData,
                v: transmute_lifetime!(v.into()),
            };
            let fn_configurator: FnConfigurator<
                '_,
                fMock<'a, T>,
                Self,
                (&'__rsa &'a T,),
                (),
                fMock<'a, T>,
                false,
                true,
                false,
            > = fn_data.add_config(args_checker, self);
            fn_configurator
        }
    }
    pub struct fStaticReceived<'a, T: Clone> {
        generics: PhantomData<(T,)>,
    }
    impl<'a, T: Clone> fStaticReceived<'a, T> {
        pub fn received<'__rsa>(self, v: impl Into<Arg<&'a T>>, times: Times) -> Self {
            let fn_data: &FnData<fMock<'a, T>, false, true, false> = get_static_fn_data("f");
            let args_checker = f_ArgsChecker::<'a, T> {
                generics: PhantomData,
                v: transmute_lifetime!(v.into()),
            };
            fn_data.verify_received(args_checker, times);
            self
        }
        pub fn no_other_calls(self) {
            let fn_data: &FnData<fMock<'a, T>, false, true, false> = get_static_fn_data("f");
            fn_data.verify_received_nothing_else([])
        }
    }
}

fn main() {
    let a = [1, 0, 1];
    let a0 = &a[0];
    let a2 = &a[2];
    f::setup::<i32>(a0)
        .returns(&101)
        .and_does(|_| println!("em kavo))"))
        .setup(Arg::Any)
        .returns_with(|(v,)| {
            println!("re-returning {v}");
            return v;
        })
        .and_does(|(v,)| println!("chevooo {v}"));
    f(a0);
    f(a2);
    f(&2);
    f(&3);
    f::received::<&i32>(3, Times::Exactly(1));
    println!("Done");
}
