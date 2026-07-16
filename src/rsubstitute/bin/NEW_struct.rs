#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use crate::__rsubstitute_generated_StructMock::Mockable;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock]
struct Struct<S1>(pub S1);

pub use __rsubstitute_generated_Struct_1_1::*;
#[allow(non_camel_case_types)]
pub mod __rsubstitute_generated_Struct_1_1 {
    #[allow(unused_imports)]
    use super::*;
    use rsubstitute::for_generated::*;
    impl<S1> Struct<S1> {
        pub fn f(&self, v: i32) -> f32
        where
            S1: Clone + Into<i32>,
        {
            (v + self.0.clone().into()) as f32
        }

        pub fn f_static(v: i32) -> f32 {
            23f32
        }
    }
    pub struct f_Call<S1>
    where
        S1: Clone + Into<i32>,
    {
        generics: ::core::marker::PhantomData<(S1, i32)>,
        v: i32,
    }
    impl<S1> IGenericsInfoProvider for f_Call<S1>
    where
        S1: Clone + Into<i32>,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }

    impl<S1> ICall for f_Call<S1>
    where
        S1: Clone + Into<i32>,
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "v",
                &self.v,
                (&ArgPrinter(transmute_lifetime!(&self.v, &i32))).debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.v,))) as *mut _ as *mut ()
        }
    }
    struct f_ArgsChecker<S1>
    where
        S1: Clone + Into<i32>,
    {
        generics: ::core::marker::PhantomData<(S1, i32)>,
        v: Arg<i32>,
    }
    impl<S1> IGenericsInfoProvider for f_ArgsChecker<S1>
    where
        S1: Clone + Into<i32>,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }

    impl<S1> IArgsChecker for f_ArgsChecker<S1>
    where
        S1: Clone + Into<i32>,
    {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &f_Call<S1> = dyn_call.downcast_ref();
            vec![transmute_lifetime!(&self.v, &Arg<i32>).check(
                "v",
                transmute_lifetime!(&call.v),
                (&ArgPrinter(transmute_lifetime!(&call.v, &i32))).debug_string(),
            )]
        }
        fn fmt_args(&self) -> String {
            format!(
                "{}",
                (&ArgPrinter(transmute_lifetime!(&&self.v, &&Arg<i32>))).debug_string()
            )
        }
    }
    pub struct f_static_Call<S1> {
        generics: ::core::marker::PhantomData<(S1, i32)>,
        v: i32,
    }
    impl<S1> IGenericsInfoProvider for f_static_Call<S1> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<S1> ICall for f_static_Call<S1> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "v",
                &self.v,
                (&ArgPrinter(transmute_lifetime!(&self.v, &i32))).debug_string(),
            )]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.v,))) as *mut _ as *mut ()
        }
    }
    struct f_static_ArgsChecker<S1> {
        generics: ::core::marker::PhantomData<(S1, i32)>,
        v: Arg<i32>,
    }
    impl<S1> IGenericsInfoProvider for f_static_ArgsChecker<S1> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<S1> IArgsChecker for f_static_ArgsChecker<S1> {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &f_static_Call<S1> = dyn_call.downcast_ref();
            vec![transmute_lifetime!(&self.v, &Arg<i32>).check(
                "v",
                transmute_lifetime!(&call.v),
                (&ArgPrinter(transmute_lifetime!(&call.v, &i32))).debug_string(),
            )]
        }
        fn fmt_args(&self) -> String {
            format!(
                "{}",
                (&ArgPrinter(transmute_lifetime!(&&self.v, &&Arg<i32>))).debug_string()
            )
        }
    }
    impl<S1> StructMock<S1> {
        fn f(&self, v: i32) -> f32
        where
            S1: Clone + Into<i32>,
        {
            let call = f_Call::<S1> {
                generics: ::core::marker::PhantomData,
                v: transmute_lifetime!(v),
            };
            let fn_data: &FnData<StructMock<S1>, true, false, false> = self
                .data
                .get_shared_fn_data("f", call.get_generics_hash_key());
            fn_data.handle(
                self,
                call,
            )
        }
        fn f_static(v: i32) -> f32 {
            let call = f_static_Call::<S1> {
                generics: ::core::marker::PhantomData,
                v: transmute_lifetime!(v),
            };
            let fn_data: &FnData<StructMock<S1>, true, false, false> =
                get_static_fn_data("f_static");
            fn_data.handle((), call)
        }
    }
}

fn main() {
    let s = Struct(1);
    let s_mock = s.mock();
    let s = s_mock.unmock();

    println!("Done");
}

#[mock(base)]
trait Trait {
    fn f(v: i32) -> i32;
}
use __rsubstitute_generated_TraitMock::*;

#[mock(base)]
fn f(v: i32) -> i32 {
    v + 10
}
