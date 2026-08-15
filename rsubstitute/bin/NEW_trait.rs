#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::Times;
use rsubstitute_core::args::Arg;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock(base)]
trait Trait<T1: Clone> {
    fn ok(&self) -> i32;
    fn ok_static() -> i32;
    fn f(&self, v: i32) {
        let ok_v = self.ok();
        println!("base f! v = {v}, self ok = {ok_v}");
    }
    fn f_static(v: i32) {
        let ok_v = Self::ok_static();
        println!("base f static! v = {v}, self ok = {ok_v}");
    }

    fn gg<T2>(&self, t1: T1, t2: T2);

    fn gg_static<T2>(t1: T1, t2: T2);
}

fn main() {
    let mut mock = TraitMock::<u8>::new();

    mock.setup()
        .ok()
        .returns_many([9, 8, 7])
        .f(1)
        .does(|_, _| println!("first mocked callback"))
        .f(2)
        .call_base()
        .and_does(|_, (v,)| println!("little extra BEFORE base call, v = {v}")) // TODO - write in docs/limitations that `and_does` is called before base call
        .f(Arg::Any)
        .does(|_, (v,)| println!("any v = {v}"));
    mock.f(1);
    mock.f(2);
    mock.f(2);
    mock.f(2);
    mock.f(3);
    TraitMock::<i32>::static_setup()
        .ok_static()
        .returns_many([99, 88, 77])
        .f_static(11)
        .does(|_| println!("first mocked static call"))
        .f_static(22)
        .call_base()
        .and_does(|(v,)| println!("little extra static BEFORE base call, v = {v}"))
        .f_static(Arg::Any)
        .does(|(v,)| println!("static any v = {v}"));
    TraitMock::<&[u8]>::static_setup()
        .f_static(22)
        .does(|_| println!("this is from &[u8]"));
    TraitMock::<i32>::f_static(11);
    TraitMock::<i32>::f_static(22);
    TraitMock::<i32>::f_static(22);
    TraitMock::<i32>::f_static(22);
    TraitMock::<&[u8]>::f_static(22);
    TraitMock::<i32>::f_static(33);

    mock.received().f(1, Times::Once);

    println!("Done");
}
