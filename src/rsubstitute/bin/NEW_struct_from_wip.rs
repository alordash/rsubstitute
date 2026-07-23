#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute::*;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

trait Trait<T1> {
    fn f<T2>(&self) -> T1;
    fn g<T3>();
    fn tself<T4>(&self);
    fn tstatic<T5>();
}
trait Gen<G1> {
    fn f<G2>(&self);
    fn g<G3>();
    fn gself<G4>(&self);
    fn gstatic<G5>();
}

// TODO - need to implement StructReceived::no_other_calls in this module, not in each `impl Struct`
// modules (same for StaticStructReceived)
#[mock]
struct Struct<S1>(pub S1);
#[mock]
impl<S1> Struct<S1> {
    pub fn f<S2>(&self) {}
    pub fn g<S3>() {}
}
#[mock]
impl Struct<i8> {
    pub fn sself<S4>(&self) {}
    pub fn sstatic<S5>() {}
}

// this `<T1>` in `impl<T1>` dictates which generics should be in `as_Trait<T1>()`
#[mock]
impl<T1> Trait<T1> for Struct<i16> {
    fn f<T2>(&self) -> T1 {
        todo!()
    }
    fn g<T3>() {}
    fn tself<T4>(&self) {}
    fn tstatic<T5>() {}
}
#[mock]
impl Trait<Box<[i64]>> for Struct<i128> {
    fn f<T2>(&self) -> Box<[i64]> {
        todo!()
    }
    fn g<T3>() {}
    fn tself<T4>(&self) {}
    fn tstatic<T5>() {}
}
#[mock]
impl<G1, S1> Gen<G1> for Struct<S1>
where
    G1: Clone,
    S1: Default,
{
    fn f<G2>(&self) {}
    fn g<G3>() {}
    fn gself<G4>(&self) {}
    fn gstatic<G5>() {}
}

fn main() {
    let s = Struct(10i16);
    let mut s_mock = s.mock();
    // TODO - need to split `as_Trait` methods
    s_mock.received().as_Trait::<i32>();
    s_mock.setup().as_Gen::<[u8; 3]>();

    println!("Done");
}
