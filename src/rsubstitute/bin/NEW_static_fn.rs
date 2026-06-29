#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
#[allow(unused_imports)]
use std::fmt::Debug;

#[mock]
fn f() {}

fn main() {
    f::setup().does(|_| println!("em kavo))")).setup().does(|_| println!("chevooo"));
    f();
    f();
    f();
    println!("Done");
}
