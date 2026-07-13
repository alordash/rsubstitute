#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]
#![feature(associated_type_defaults)]

#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;
use std::fmt::Debug;

#[mock(base)]
trait Trait {
    const CONST: usize = 43;

    type InputType<TAmogus: Clone>: Clone + Debug
    = i32
    where
        Self: Clone;

    type OutputType<TT>: Clone + Sized + Default
    = u8
    where
        Self: Sized,
        TT: Clone;

    fn get_const(&self) -> usize {
        Self::CONST
    }

    fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
    where
        Self: Clone + Sized,
        TT: ToString;
}

fn main() {
    println!("Dones");
}
