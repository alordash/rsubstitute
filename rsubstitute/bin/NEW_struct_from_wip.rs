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

#[mock]
struct Struct<S1> {
    pub s1: S1,
}
#[mock]
impl<S1> Struct<S1> {
    pub fn new(s1: S1) -> Self {
        Self { s1 }
    }
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
    Struct::static_setup().new(Arg::Any).returns(Struct {
        s1: 3i16,
        __rs_data: Default::default(),
    });
    let mut s = Struct::new(10i16);
    println!("s.s1 = {}", s.s1);
    s.received().as_Trait::<i32>().no_other_calls();
    s.setup().as_Gen::<[u8; 3]>();
    s.received().no_other_calls();
    Struct::<i128>::static_setup().as_Gen::<[[u8; 2]; 1]>();
    Struct::<i128>::static_received()
        .as_Gen::<[[u8; 2]; 1]>()
        .no_other_calls();

    println!("Done");
}
