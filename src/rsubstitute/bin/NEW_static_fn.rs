#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::args::Arg;
use rsubstitute_core::Times;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock(base)]
fn f<'a, T: Clone>(v: &'a T) -> i32 {
    121
}

use accept_many_ref::accept_many_ref;
#[allow(non_camel_case_types)]
pub mod accept_many_ref {
    #[allow(unused_imports)]
    use super::*;
    use rsubstitute::for_generated::*;
    pub fn accept_many_ref<'a, 'b>(r: &'a &'b &'a &i32, _em: &()) -> &'a &'b &'a &'b i32 {
        let fn_data: &FnData<accept_many_refMock<'a, 'b>, true, false, false> =
            get_static_fn_data("accept_many_ref");
        fn_data.handle(
            &accept_many_refMock::<'a, 'b> {
                generics: PhantomData,
            },
            accept_many_ref_Call::<'a, 'b> {
                generics: PhantomData,
                r: transmute_lifetime!(r),
                _em: transmute_lifetime!(_em),
            },
        )
    }
    pub fn setup<'__rsa, 'a, 'b>(
        r: impl Into<Arg<&'a &'b &'a &'__rsa i32>>,
        _em: impl Into<Arg<&'__rsa ()>>,
    ) -> FnConfigurator<
        '__rsa,
        accept_many_refMock<'a, 'b>,
        accept_many_refStaticSetup<'a, 'b>,
        (&'__rsa &'a &'b &'a &'__rsa i32, &'__rsa &'__rsa ()),
        &'a &'b &'a &'b i32,
        accept_many_refMock<'a, 'b>,
        true,
        false,
        false,
    > {
        let fn_data: &FnData<accept_many_refMock<'a, 'b>, true, false, false> =
            get_static_fn_data("accept_many_ref");
        fn_data.reset();
        accept_many_refStaticSetup::<'a, 'b> {
            generics: PhantomData,
        }
        .setup(r, _em)
    }
    pub fn received<'__rsa, 'a, 'b>(
        r: impl Into<Arg<&'a &'b &'a &'__rsa i32>>,
        _em: impl Into<Arg<&'__rsa ()>>,
        times: Times,
    ) -> accept_many_refStaticReceived<'a, 'b>
    // TODO - this is manual fix
    where
        '__rsa: 'a + 'b,
        'a: '__rsa,
        'b: '__rsa,
    {
        accept_many_refStaticReceived::<'a, 'b> {
            generics: PhantomData,
        }
        .received(r, _em, times)
    }
    pub struct accept_many_ref_Call<'a, 'b> {
        generics: PhantomData<(&'a (), &'b (), &'a &'b &'a *const i32, *const ())>,
        r: *const *const *const *const i32,
        _em: *const (),
    }
    impl<'a, 'b> IGenericsInfoProvider for accept_many_ref_Call<'a, 'b> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<'a, 'b> ICall for accept_many_ref_Call<'a, 'b> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new(
                    "r",
                    &self.r,
                    (&ArgPrinter(transmute_lifetime!(&self.r, &&'a &'b &'a &i32))).debug_string(),
                ),
                ArgInfo::new(
                    "_em",
                    &self._em,
                    (&ArgPrinter(transmute_lifetime!(&self._em, &&()))).debug_string(),
                ),
            ]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.r, &self._em))) as *mut _ as *mut ()
        }
    }
    struct accept_many_ref_ArgsChecker<'a, 'b> {
        generics: PhantomData<(&'a (), &'b (), &'a &'b &'a *const i32, *const ())>,
        r: Arg<*const *const *const *const i32>,
        _em: Arg<*const ()>,
    }
    impl<'a, 'b> IGenericsInfoProvider for accept_many_ref_ArgsChecker<'a, 'b> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<'a, 'b> IArgsChecker for accept_many_ref_ArgsChecker<'a, 'b> {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &accept_many_ref_Call<'a, 'b> = dyn_call.downcast_ref();
            vec![
                transmute_lifetime!(&self.r, &Arg<&'a &'b &'a &i32>).check_ref(
                    "r",
                    transmute_lifetime!(&call.r),
                    (&ArgPrinter(transmute_lifetime!(&call.r, &&'a &'b &'a &i32))).debug_string(),
                ),
                transmute_lifetime!(&self._em, &Arg<&()>).check_ref(
                    "_em",
                    transmute_lifetime!(&call._em),
                    (&ArgPrinter(transmute_lifetime!(&call._em, &&()))).debug_string(),
                ),
            ]
        }
        fn fmt_args(&self) -> String {
            format!(
                "{}, {}",
                (&ArgPrinter(transmute_lifetime!(&&self.r, &&Arg<&'a &'b &'a &i32>)))
                    .debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self._em, &&Arg<&()>))).debug_string()
            )
        }
    }
    pub struct accept_many_refMock<'a, 'b> {
        generics: PhantomData<(&'a (), &'b (), &'a &'b &'a *const i32, *const ())>,
    }
    pub struct accept_many_refStaticSetup<'a, 'b> {
        generics: PhantomData<(&'a (), &'b (), &'a &'b &'a *const i32, *const ())>,
    }
    impl<'a, 'b> accept_many_refStaticSetup<'a, 'b> {
        pub fn setup<'__rsa>(
            &self,
            r: impl Into<Arg<&'a &'b &'a &'__rsa i32>>,
            _em: impl Into<Arg<&'__rsa ()>>,
        ) -> FnConfigurator<
            '_,
            accept_many_refMock<'a, 'b>,
            Self,
            (&'__rsa &'a &'b &'a &'__rsa i32, &'__rsa &'__rsa ()),
            &'a &'b &'a &'b i32,
            accept_many_refMock<'a, 'b>,
            true,
            false,
            false,
        > {
            let fn_data: &FnData<accept_many_refMock<'a, 'b>, true, false, false> =
                get_static_fn_data("accept_many_ref");
            let args_checker = accept_many_ref_ArgsChecker::<'a, 'b> {
                generics: PhantomData,
                r: transmute_lifetime!(r.into()),
                _em: transmute_lifetime!(_em.into()),
            };
            let fn_configurator: FnConfigurator<
                '_,
                accept_many_refMock<'a, 'b>,
                Self,
                (&'__rsa &'a &'b &'a &'__rsa i32, &'__rsa &'__rsa ()),
                &'a &'b &'a &'b i32,
                accept_many_refMock<'a, 'b>,
                true,
                false,
                false,
            > = fn_data.add_config(args_checker, self);
            fn_configurator
        }
    }
    pub struct accept_many_refStaticReceived<'a, 'b> {
        generics: PhantomData<(&'a (), &'b (), &'a &'b &'a *const i32, *const ())>,
    }
    impl<'a, 'b> accept_many_refStaticReceived<'a, 'b> {
        pub fn received<'__rsa>(
            self,
            r: impl Into<Arg<&'a &'b &'a &'__rsa i32>>,
            _em: impl Into<Arg<&'__rsa ()>>,
            times: Times,
        ) -> Self
        // TODO - this is manual fix
        where
            '__rsa: 'a + 'b,
            'a: '__rsa,
            'b: '__rsa,
        {
            let fn_data: &FnData<accept_many_refMock<'a, 'b>, true, false, false> =
                get_static_fn_data("accept_many_ref");
            let args_checker = accept_many_ref_ArgsChecker::<'a, 'b> {
                generics: PhantomData,
                r: transmute_lifetime!(r.into()),
                _em: transmute_lifetime!(_em.into()),
            };
            fn_data.verify_received(args_checker, times);
            self
        }
        pub fn no_other_calls(self) {
            let fn_data: &FnData<accept_many_refMock<'a, 'b>, true, false, false> =
                get_static_fn_data("accept_many_ref");
            fn_data.verify_received_nothing_else([])
        }
    }
}

fn main() {
    let a = [1, 0, 1];
    let a0 = &a[0];
    let a2 = &a[2];
    f::setup::<i32>(a0)
        .returns(101)
        .and_does(|_| println!("em kavo))"))
        .setup(Arg::Any)
        .returns_with(|(v,)| {
            println!("re-returning {v}");
            return **v + 1;
        })
        .and_does(|(v,)| println!("chevooo {v}"));
    f(a0);
    f(a2);
    f(&2);
    f(&3);
    f::received::<i32>(&3, Times::Exactly(1));
    println!("Done");
}
