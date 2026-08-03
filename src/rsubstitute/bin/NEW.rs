#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute::Mockable;
use rsubstitute_core::args::Arg;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::ops::Deref;
use rsubstitute_core::Times;

#[mock]
fn _f(t: impl Trait) {
    t.flex();
}

#[mock]
fn f(t: impl Trait) {
    t.flex();
}

#[mock]
fn g(t: impl Trait) {}

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
    f::received(Arg::Any, Times::Once);
    
    let _s = Struct(235325);
    _f::setup(Arg::is(|p: &Box<dyn Trait>| {
        dbg!(p.v(), _s.0);
        let result = p.v() == _s.0;
        return result;
    })).does(|a| {a.0.flex()});
    _f(_s);
    _f::received(Arg::Any, Times::Once);

    println!("Done");
}
