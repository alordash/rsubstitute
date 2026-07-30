#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute::Mockable;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::ops::Deref;
use std::thread;

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
struct Struct;

#[mock(base)]
impl Struct {
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
        Struct::static_setup()
            .work()
            .does(|_| println!("thread Struct::work mocked!"));
        Struct::work()
    })
    .join();
    Struct::work();

    println!("Done");
}
