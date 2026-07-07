#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock]
trait Trait {
    fn f(&self);
}

fn main() {
    println!("Done");
}
