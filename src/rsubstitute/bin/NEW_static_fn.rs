#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::args::Arg;
use rsubstitute_core::Times;
use rsubstitute_proc_macro::mock;

#[mock(base)]
fn f(v: &i32) {}

fn main() {
    let a = [1, 0, 1];
    let a0 = &a[0];
    let a2 = &a[2];
    f::setup(a0)
        .does(|_| println!("em kavo))"))
        .setup(Arg::Any)
        .does(|(v, )| println!("chevooo {v}"));
    f(a0);
    f(a2);
    f(&2);
    f(&3);
    f::received(&3, Times::Exactly(1));
    println!("Done");
}
