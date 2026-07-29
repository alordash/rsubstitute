#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::Times;
use rsubstitute_core::args::Arg;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock]
fn f() {}

fn g(r#fn: fn()) {
    r#fn()
}

fn main() {
    f::setup().does(|_| println!("mocked!"));
    g(f);
    
    println!("Done");
}
