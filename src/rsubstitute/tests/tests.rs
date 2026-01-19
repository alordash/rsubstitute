#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_proc_macro::mock;
use std::fmt::Debug;
use std::sync::Arc;

trait IFoo: Debug {
    fn get_value(&self) -> i32;
}

#[mock]
trait MyTrait {
    fn work(&self, value: i32);

    fn another_work(
        &self,
        string: &str,
        something: &&[u8],
        dyn_obj: &dyn IFoo,
        arc: Arc<dyn IFoo>,
    ) -> Vec<u8>;
}

use rsubstitute::*;

#[test]
fn work() {
    let my_trait_mock = MyTraitMock::new();
    my_trait_mock
        .setup
        .work(Arg::Any)
        .does(|| println!("work 1"));

    my_trait_mock.work(20);

    my_trait_mock.received.work(Arg::Eq(20), Times::Once);
}

#[test]
#[cfg(test)]
fn another_work() {
    let my_trait_mock = MyTraitMock::new();
    // my_trait_mock.another_work(Arg::Any);
}
