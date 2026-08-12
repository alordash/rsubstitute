#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute::Mockable;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::marker::PhantomData;
use std::ops::Deref;

// #[rsubstitute::mock]
// fn f() -> impl Trait {
//     Struct(123)
// }
//
// #[mock]
// trait Trait {
//     fn flex(&self);
//     fn v(&self) -> i32;
// }
//
// #[derive(Clone, Debug)]
// struct Struct(i32);
// impl Trait for Struct {
//     fn flex(&self) {
//         println!("base struct flex")
//     }
//
//     fn v(&self) -> i32 {
//         self.0
//     }
// }
//
// #[mock]
// fn kak() -> (impl Trait, impl Trait) {
//     (Struct(1), Struct(2))
// }

#[derive(Clone)]
struct Consumer<'a> {
    phantom: PhantomData<&'a ()>,
}

#[mock(base)]
fn consume(consumer: Consumer<'_>) {}

#[mock]
fn option_ref(v: Option<&i32>) {}

#[mock]
fn arg_impl(v: impl IntoIterator<Item = i32, IntoIter = Vec<i32>>) {}

#[mock]
struct S<T> {
    t: T,
}

#[mock]
impl<T> S<T> {
    pub(crate) fn new(t: T) -> Self {
        S { t }
    }
}

struct MyI32;
impl PartialEq<MyI32> for i32 {
    fn eq(&self, _: &MyI32) -> bool {
        todo!()
    }
}

#[mock(base)]
impl<T: PartialEq<U>, U> PartialEq<U> for S<T> {
    fn eq(&self, _: &U) -> bool {
        todo!()
    }
}

fn main() {
    let mut s = S {
        t: 3,
        __rs_data: Default::default(),
    };
    s.setup()
        .as_PartialEq()
        .eq(rsubstitute::Arg::<&MyI32>::Any)
        .returns(true);

    // let s = Struct(63);
    // f::setup().returns(Box::new(s));
    // let result = f();
    // dbg!(result.v());
    //
    // kak::setup().returns((Box::new(Struct(10)), Box::new(Struct(20))));
    // let (s1, s2) = kak();
    // dbg!(s1.v(), s2.v());

    println!("Done");
}

// trait Chto {
//     fn k() -> impl Trait {
//         Struct(-1)
//     }
// }
