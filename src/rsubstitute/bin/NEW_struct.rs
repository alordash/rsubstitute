#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use crate::__rsubstitute_generated_StructMock::Mockable;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock]
struct Struct<S1>(pub S1);

fn main() {
    let s = Struct(1);
    let s_mock = s.mock();
    let s = s_mock.unmock();

    println!("Done");
}
