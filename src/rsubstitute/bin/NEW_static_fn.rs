#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::args::Arg;
use rsubstitute_core::Times;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

// TODO - need to fix Call and ArgsChecker structs paths
#[mock(base)]
fn f<T: Clone>(v: T) -> T {
    v
}

fn main() {
    let a = [1, 0, 1];
    let a0 = &a[0];
    let a2 = &a[2];
    f::setup::<&i32>(a0)
        .returns(&101)
        .and_does(|_| println!("em kavo))"))
        .setup(Arg::Any)
        .returns_with(|(v,)| {
            println!("re-returning {v}");
            return v;
        })
        .and_does(|(v,)| println!("chevooo {v}"));
    f(a0);
    f(a2);
    f(&2);
    f(&3);
    f::received::<&i32>(&3, Times::Exactly(1));
    println!("Done");
}
