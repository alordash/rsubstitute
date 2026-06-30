#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::args::Arg;
use rsubstitute_core::Times;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
#[allow(unused_imports)]
use std::fmt::Debug;

#[mock]
fn f(v: i32) {}

fn main() {
    f::setup(1)
        .does(|_| println!("em kavo))"))
        .setup(Arg::Any)
        .does(|(v,)| println!("chevooo {v}"));
    f(1);
    f(2);
    f(3);
    f::received(3, Times::Exactly(1));
    println!("Done");
}
