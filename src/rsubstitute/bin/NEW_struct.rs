#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute::*;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::ops::Deref;

pub use __rsubstitute_generated_StructMock::{Struct, StructMock};
mod __rsubstitute_generated_StructMock {
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone)]
    pub struct Struct<S1: Clone>(pub S1);
    impl<S1: Clone> ::rsubstitute::Mockable for Struct<S1> {
        type Mock = StructMock<S1>;
        fn mock(self) -> Self::Mock {
            StructMock::<S1> {
                generics: ::core::marker::PhantomData,
                data: ::core::default::Default::default(),
                mockable: Box::new(self),
            }
        }
        type StaticSetup = StructStaticSetup<S1>;
        fn static_setup() -> Self::StaticSetup {
            Self::StaticSetup {
                generics: ::core::marker::PhantomData,
            }
        }
        type StaticReceived = StructStaticReceived<S1>;
        fn static_received() -> Self::StaticReceived {
            Self::StaticReceived {
                generics: ::core::marker::PhantomData,
            }
        }
    }
    #[derive(Clone)]
    pub struct StructMock<S1: Clone> {
        pub generics: ::core::marker::PhantomData<(S1,)>,
        pub data: ::rsubstitute::for_generated::SharedMockData,
        pub mockable: Box<Struct<S1>>,
    }
    impl<S1: Clone> StructMock<S1> {
        pub fn unmock(self) -> Struct<S1> {
            *self.mockable
        }
        pub fn setup(&mut self) -> StructSetup<S1> {
            StructSetup::<S1> {
                generics: ::core::marker::PhantomData,
                data: self.data.clone(),
            }
        }
        pub fn received(&mut self) -> StructReceived<S1> {
            StructReceived::<S1> {
                generics: ::core::marker::PhantomData,
                data: self.data.clone(),
            }
        }
    }
    impl<S1: Clone> core::ops::Deref for StructMock<S1> {
        type Target = Struct<S1>;
        fn deref(&self) -> &Self::Target {
            self.mockable.as_ref()
        }
    }
    impl<S1: Clone> core::ops::DerefMut for StructMock<S1> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.mockable.as_mut()
        }
    }
    pub struct StructSetup<S1: Clone> {
        pub generics: ::core::marker::PhantomData<(S1,)>,
        pub data: ::rsubstitute::for_generated::SharedMockData,
    }
    impl<S1: Clone> ::core::clone::Clone for StructSetup<S1> {
        #[inline]
        fn clone(&self) -> StructSetup<S1> {
            StructSetup::<S1> {
                generics: ::core::clone::Clone::clone(&self.generics),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
    }
    pub struct StructReceived<S1: Clone> {
        pub generics: ::core::marker::PhantomData<(S1,)>,
        pub data: ::rsubstitute::for_generated::SharedMockData,
    }
    impl<S1: Clone> ::core::clone::Clone for StructReceived<S1> {
        #[inline]
        fn clone(&self) -> StructReceived<S1> {
            StructReceived::<S1> {
                generics: ::core::clone::Clone::clone(&self.generics),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
    }
    impl<S1: Clone> StructReceived<S1> {
        pub fn no_other_calls(&self) {
            rsubstitute::for_generated::IMockData::verify_received_nothing_else(&self.data)
        }
    }
    pub struct StructStaticSetup<S1: Clone> {
        pub generics: ::core::marker::PhantomData<(S1,)>,
    }
    impl<S1: Clone> ::core::clone::Clone for StructStaticSetup<S1> {
        #[inline]
        fn clone(&self) -> StructStaticSetup<S1> {
            StructStaticSetup::<S1> {
                generics: ::core::clone::Clone::clone(&self.generics),
            }
        }
    }
    pub struct StructStaticReceived<S1: Clone> {
        pub generics: ::core::marker::PhantomData<(S1,)>,
    }
    impl<S1: Clone> ::core::clone::Clone for StructStaticReceived<S1> {
        #[inline]
        fn clone(&self) -> StructStaticReceived<S1> {
            StructStaticReceived::<S1> {
                generics: ::core::clone::Clone::clone(&self.generics),
            }
        }
    }
    impl<S1: Clone> StructStaticReceived<S1> {
        pub fn no_other_calls(&self) {
            ::rsubstitute::for_generated::verify_static_fn_received_nothing_else::<StructMock<S1>>()
        }
    }
}

#[mock(base)]
impl<S1: Clone> Struct<S1> {
    // pub fn f(&self, v: i32) -> f32
    // where
    //     S1: Clone + Into<i32>,
    // {
    //     (v + self.0.clone().into()) as f32
    // }
    //
    // pub fn f_static(v: i32) -> f32 {
    //     23f32
    // }

    pub fn transform(s: Self) -> Self {
        let Self(s0) = s;
        Self(s0)
    }
}

fn main() {
    let s = Struct(10);
    let mut s_mock = s.mock();
    // s_mock
    //     .setup()
    //     .f(4)
    //     .returns(22f32)
    //     .and_does(|(v,)| println!("mocked for 4, v = {v}"))
    //     .f(3)
    //     .call_base()
    //     .f(Arg::Any)
    //     .returns_with(|(v,)| *v as f32 + 1f32)
    //     .and_does(|(v,)| println!("mocked for any, v = {v}"));
    // dbg!(s_mock.deref().f(1));
    // dbg!(s_mock.f(1));
    // dbg!(s_mock.f(2));
    // dbg!(s_mock.f(3));
    // dbg!(s_mock.f(4));
    // dbg!(s_mock.f(5));
    // Struct::<i32>::static_setup()
    //     .f_static(2)
    //     .returns_many([5f32, 1112f32])
    //     .and_does(|(v,)| println!("Mocked static, v = {v}"));
    // dbg!(Struct::<i32>::f_static(2));
    // Struct::<i32>::static_received()
    //     .f_static(2, Times::Once)
    //     .no_other_calls();
    let s = s_mock.unmock();

    println!("Done");
}

// #[mock(base)]
// trait Trait {
//     fn f(v: i32) -> i32;
// }
// use __rsubstitute_generated_TraitMock::*;
//
// #[mock(base)]
// fn f(v: i32) -> i32 {
//     v + 10
// }
