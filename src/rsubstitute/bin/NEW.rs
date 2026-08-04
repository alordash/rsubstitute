#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute::Mockable;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::ops::Deref;

#[rsubstitute::mock]
fn f() -> impl Trait {
    Struct(123)
}

#[mock]
trait Trait {
    fn flex(&self);
    fn v(&self) -> i32;
}

#[derive(Clone, Debug)]
struct Struct(i32);
impl Trait for Struct {
    fn flex(&self) {
        println!("base struct flex")
    }

    fn v(&self) -> i32 {
        self.0
    }
}

#[mock]
fn kak() -> (impl Trait, impl Trait) {
    (Struct(1), Struct(2))
}

fn main() {
    let s = Struct(63);
    f::setup().returns(Box::new(s));
    let result = f();
    dbg!(result.v());

    kak::setup().returns((Box::new(Struct(10)), Box::new(Struct(20))));
    let (s1, s2) = kak();
    dbg!(s1.v(), s2.v());

    println!("Done");
}

trait Chto {
    fn k() -> impl Trait {
        Struct(-1)
    }
}
