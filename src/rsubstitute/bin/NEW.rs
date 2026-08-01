#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute::Mockable;
use rsubstitute_core::args::Arg;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::ops::Deref;

fn _f(t: impl Trait) {
    t.flex();
}

pub fn f(t: impl Trait) {
    use f::*;
    let q: Box<dyn Trait> = Box::new(t);
    let call = f::f_Call {
        generics: ::core::marker::PhantomData,
        t: ::rsubstitute::transmute_lifetime!(q),
    };
    let fn_data: &::rsubstitute::for_generated::FnData<fMock, false, false, false> =
        ::rsubstitute::for_generated::get_static_fn_data("f");
    fn_data.handle((), call)
}
#[allow(non_camel_case_types)]
mod f {
    #[allow(unused_imports)]
    use super::*;
    use rsubstitute::for_generated::*;
    pub fn setup<'__rsa>(
        t: impl Into<Arg<Box<dyn Trait>>> + '__rsa,
    ) -> FnConfigurator<
        '__rsa,
        fMock,
        fStaticSetup,
        (&'__rsa Box<dyn Trait>,),
        (),
        fMock,
        false,
        false,
        false,
    > {
        ::rsubstitute::for_generated::clear_static_fn_data::<fMock>();
        fStaticSetup {
            generics: ::core::marker::PhantomData,
        }
        .setup(t)
    }
    pub fn received<'__rsa>(
        t: impl Into<Arg<Box<dyn Trait>>> + '__rsa,
        times: Times,
    ) -> ::rsubstitute::for_generated::ArgRefsBinder<fStaticReceived, (&'__rsa dyn Trait,)> {
        fStaticReceived {
            generics: ::core::marker::PhantomData,
        }
        .received(t, times)
    }
    pub struct f_Call {
        pub generics: ::core::marker::PhantomData<(dyn Trait,)>,
        pub(super) t: Box<dyn Trait>,
    }
    impl IGenericsInfoProvider for f_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl ICall for f_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "t",
                &self.t,
                (&ArgPrinter(::rsubstitute::transmute_lifetime!(&self.t, &Box<dyn Trait>)))
                    .debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.t,))) as *mut _ as *mut ()
        }
    }
    struct f_ArgsChecker {
        pub generics: ::core::marker::PhantomData<(dyn Trait,)>,
        t: Arg<Box<dyn Trait>>,
    }
    impl IGenericsInfoProvider for f_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl IArgsChecker for f_ArgsChecker {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &f_Call = dyn_call.downcast_ref();
            vec![
                ::rsubstitute::transmute_lifetime!(&self.t, &Arg<Box<dyn Trait>>).check(
                    "t",
                    ::rsubstitute::transmute_lifetime!(&call.t),
                    (&ArgPrinter(::rsubstitute::transmute_lifetime!(&call.t, &Box<dyn Trait>)))
                        .debug_string(),
                ),
            ]
        }
        fn fmt_args(&self) -> String {
            format!(
                "{}",
                (&ArgPrinter(::rsubstitute::transmute_lifetime!(
                    &&self.t,
                    &&Arg<dyn Trait>
                )))
                    .debug_string()
            )
        }
    }
    pub struct fMock {
        pub generics: ::core::marker::PhantomData<(dyn Trait,)>,
    }
    pub struct fStaticSetup {
        pub generics: ::core::marker::PhantomData<(dyn Trait,)>,
    }
    impl fStaticSetup {
        pub fn setup<'__rsa>(
            &self,
            t: impl Into<Arg<Box<dyn Trait>>>,
        ) -> FnConfigurator<
            '_,
            fMock,
            Self,
            (&'__rsa Box<dyn Trait>,),
            (),
            fMock,
            false,
            false,
            false,
        > {
            let args_checker = f_ArgsChecker {
                generics: ::core::marker::PhantomData,
                t: ::rsubstitute::transmute_lifetime!(t.into()),
            };
            let fn_data: &::rsubstitute::for_generated::FnData<fMock, false, false, false> =
                ::rsubstitute::for_generated::get_static_fn_data("f");
            let fn_configurator: FnConfigurator<
                '_,
                fMock,
                Self,
                (&'__rsa dyn Trait,),
                (),
                fMock,
                false,
                false,
                false,
            > = fn_data.add_config(args_checker, self);
            ::rsubstitute::transmute_lifetime!(fn_configurator)
        }
    }
    pub struct fStaticReceived {
        pub generics: ::core::marker::PhantomData<(dyn Trait,)>,
    }
    impl ::core::clone::Clone for fStaticReceived {
        #[inline]
        fn clone(&self) -> fStaticReceived {
            fStaticReceived {
                generics: ::core::clone::Clone::clone(&self.generics),
            }
        }
    }
    impl fStaticReceived {
        pub fn received<'__rsa>(
            &self,
            t: impl Into<Arg<Box<dyn Trait>>>,
            times: Times,
        ) -> ::rsubstitute::for_generated::ArgRefsBinder<Self, (&'__rsa dyn Trait,)> {
            let args_checker = f_ArgsChecker {
                generics: ::core::marker::PhantomData,
                t: ::rsubstitute::transmute_lifetime!(t.into()),
            };
            let fn_data: &::rsubstitute::for_generated::FnData<fMock, false, false, false> =
                ::rsubstitute::for_generated::get_static_fn_data("f");
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn no_other_calls(&self) {
            ::rsubstitute::for_generated::verify_static_fn_received_nothing_else::<fMock>()
        }
    }
}

trait Trait {
    fn flex(&self);
    fn v(&self) -> i32;
}

#[derive(Clone)]
struct Struct(i32);
impl Trait for Struct {
    fn flex(&self) {
        println!("base struct flex")
    }

    fn v(&self) -> i32 {
        self.0
    }
}

fn main() {
    let s = Struct(63);
    f::setup(Arg::is(|p: &Box<dyn Trait>| {
        dbg!(p.v(), s.0);
        let result = p.v() == s.0;
        return result;
    }))
    .does(|a| {
        a.0.flex();
    });
    f(s);

    println!("Done");
}
