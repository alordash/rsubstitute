#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::Times;
use rsubstitute_core::args::Arg;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock(base)]
fn f<'a, T: Clone>(v: &'a T) -> i32 {
    121
}

#[mock(base)]
fn flex<'a, 'b>(v: &'a &'b &()) {}

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

struct Q;
impl Q {
    fn kavo(&self) {}
    fn chevo(&self) {
        self.kavo();
    }
}

struct QMock;
impl QMock {
    fn __base_chevo(__rsa_self: &Q) {
        __rsa_self.kavo()
    }
}

trait Trait<T> {
    type MyType;

    fn flex(&self) -> Self::MyType;
}

impl<T> Trait<T> for Q {
    type MyType = i32;

    fn flex(&self) -> Self::MyType {
        todo!()
    }
}

impl Q {
    fn trait_usage<T>(&self) -> <Self as Trait<T>>::MyType {
        <Q as Trait<T>>::flex(self)
    }
}

trait F {
    fn ff(&self) {}
}
impl F for Q {}
impl Q {
    fn use_f(&self) {
        Q::ff(self);
    }
}
