#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::ops::Deref;

#[mock]
trait Trait {
    fn work(&self);
}
impl Trait for Box<dyn Trait> {
    fn work(&self) {
        self.deref().work();
    }
}

#[mock]
struct S;

#[mock(base)]
impl S {
    fn kavo(&self) -> impl Trait {
        TraitMock::new()
    }
}

#[mock(base)]
fn kavo() -> impl Trait {
    TraitMock::new()
}

pub(crate) fn f() -> impl Trait {
    pub use f::*;
    let call = f_Call {
        generics: ::core::marker::PhantomData,
    };
    let fn_data: &::rsubstitute::for_generated::FnData<fMock, true, false, false> =
        ::rsubstitute::for_generated::get_static_fn_data("f");
    fn_data.handle::<_, _, Box<dyn Trait>>((), call)
}
#[allow(non_camel_case_types)]
mod f {
    #[allow(unused_imports)]
    use super::*;
    use rsubstitute::for_generated::*;
    pub fn setup<'__rsa>()
    -> FnConfigurator<'__rsa, fMock, fStaticSetup, (), Box<dyn Trait>, fMock, true, false, false>
    {
        ::rsubstitute::for_generated::clear_static_fn_data::<fMock>();
        fStaticSetup {
            generics: ::core::marker::PhantomData,
        }
        .setup()
    }
    pub fn received<'__rsa>(
        times: Times,
    ) -> ::rsubstitute::for_generated::ArgRefsBinder<fStaticReceived, ()> {
        fStaticReceived {
            generics: ::core::marker::PhantomData,
        }
        .received(times)
    }
    pub struct f_Call {
        pub generics: ::core::marker::PhantomData<()>,
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
            vec![]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    struct f_ArgsChecker {
        pub generics: ::core::marker::PhantomData<()>,
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
            vec![]
        }
        fn fmt_args(&self) -> String {
            format!("")
        }
    }
    pub struct fMock {
        pub generics: ::core::marker::PhantomData<()>,
    }
    pub struct fStaticSetup {
        pub generics: ::core::marker::PhantomData<()>,
    }
    impl fStaticSetup {
        pub fn setup<'__rsa>(
            &self,
        ) -> FnConfigurator<'_, fMock, Self, (), Box<dyn Trait>, fMock, true, false, false>
        {
            let args_checker = f_ArgsChecker {
                generics: ::core::marker::PhantomData,
            };
            let fn_data: &::rsubstitute::for_generated::FnData<fMock, true, false, false> =
                ::rsubstitute::for_generated::get_static_fn_data("f");
            let fn_configurator: FnConfigurator<
                '_,
                fMock,
                Self,
                (),
                Box<dyn Trait>,
                fMock,
                true,
                false,
                false,
            > = fn_data.add_config(args_checker, self);
            ::rsubstitute::transmute_lifetime!(fn_configurator)
        }
    }
    pub struct fStaticReceived {
        pub generics: ::core::marker::PhantomData<()>,
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
            times: Times,
        ) -> ::rsubstitute::for_generated::ArgRefsBinder<Self, ()> {
            let args_checker = f_ArgsChecker {
                generics: ::core::marker::PhantomData,
            };
            let fn_data: &::rsubstitute::for_generated::FnData<fMock, true, false, false> =
                ::rsubstitute::for_generated::get_static_fn_data("f");
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn no_other_calls(&self) {
            ::rsubstitute::for_generated::verify_static_fn_received_nothing_else::<fMock>()
        }
    }
}

fn main() {
    let mut t = TraitMock::new();
    t.setup().work().does(|_, _| println!("mocked work!"));
    f::setup().returns(Box::new(t));
    let q = f();
    q.work();

    println!("Done");
}
