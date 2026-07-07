#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::args::Arg;
use rsubstitute_core::Times;
#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock(base)]
trait Trait {
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
}

fn main() {
    let mut mock = TraitMock::new();

    mock.setup()
        .ok()
        .returns_many([9, 8, 7])
        .f(1)
        .does(|_| println!("first mocked callback"))
        .f(2)
        .call_base()
        .and_does(|(v,)| println!("little extra BEFORE base call, v = {v}")) // TODO - write in docs/limitations that `and_does` is called before base call
        .f(Arg::Any)
        .does(|(v,)| println!("any v = {v}"));
    mock.f(1);
    mock.f(2);
    mock.f(2);
    mock.f(2);
    mock.f(3);
    TraitMock::static_setup()
        .ok_static()
        .returns_many([99, 88, 77])
        .f_static(11)
        .does(|_| println!("first mocked static call"))
        .f_static(22)
        .call_base()
        .and_does(|(v,)| println!("little extra static BEFORE base call, v = {v}"))
        .f_static(Arg::Any)
        .does(|(v,)| println!("static any v = {v}"));
    TraitMock::f_static(11);
    TraitMock::f_static(22);
    TraitMock::f_static(22);
    TraitMock::f_static(22);
    TraitMock::f_static(33);
    
    mock.received().f(1, Times::Once);

    println!("Done");
}
