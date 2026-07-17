#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use rsubstitute::*;

#[mock]
struct Struct<S1>(pub S1);

#[mock]
impl<S1> Struct<S1> {
    pub fn f(&self, v: i32) -> f32
    where
        S1: Clone + Into<i32>,
    {
        (v + self.0.clone().into()) as f32
    }

    pub fn f_static(v: i32) -> f32 {
        23f32
    }
}

fn main() {
    let s = Struct(1);
    let s_mock = s.mock();
    let s = s_mock.unmock();

    println!("Done");
}

#[mock(base)]
trait Trait {
    fn f(v: i32) -> i32;
}
use __rsubstitute_generated_TraitMock::*;

#[mock(base)]
fn f(v: i32) -> i32 {
    v + 10
}
