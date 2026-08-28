#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute::Mockable;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::ops::Deref;
use std::thread;

#[mock(base)]
trait Kavo {
    fn by_box(self: Box<Self>) {}
}

#[mock]
trait Trait {
    fn work() {
        println!("Default trait work impl");
    }
}

fn f<T: Trait>() {
    T::work();
}

#[mock]
struct Struct {
    pub v: i32,
}

#[mock(base)]
impl Struct {
    pub fn new(v: i32) -> Self {
        Self { v }
    }

    fn struct_refs(&self) {
        let s = Struct { v: 1 };
        let Struct { v: a } = s;

        let s = Struct { v: 1 };
        let Struct { v: b } = s;
    }

    pub fn f(&self) {
        println!("Default struct f impl");
    }

    pub fn work() {
        println!("Default struct work impl");
    }
}

fn main() {
    f::<TraitMock>();
    TraitMock::static_setup()
        .work()
        .does(|_| println!("static Trait::work mocked!"));
    f::<TraitMock>();
    f::<TraitMock>();
    TraitMock::static_setup()
        .work()
        .does(|_| println!("new Trait::work mocked!"));
    f::<TraitMock>();

    Struct::work();
    Struct::static_setup()
        .work()
        .does(|_| println!("static Struct::work mocked!"));
    Struct::work();
    Struct::work();
    Struct::static_setup()
        .work()
        .does(|_| println!("new Struct::work mocked!"));
    Struct::work();

    thread::spawn(|| {
        Struct::work();
        Struct::static_setup()
            .work()
            .does(|_| println!("thread Struct::work mocked!"));
        Struct::work();
    })
    .join();
    Struct::work();

    let mut s = Struct::new(32);
    s.f();
    s.setup()
        .f()
        .does(|s_ref, _| println!("mocked Struct::f! v = {}", s_ref.v));
    // TODO - maybe it's possible to add `does` overload that accepts only args without mock itself
    // attempt: https://github.com/alordash/rsubstitute/commit/07e5c2f1856e9089b4baa9a0d06e44fc263dcb60
    // s.setup().f().does(|_| println!("not mock arg"));
    s.f();

    println!("Done");
}
