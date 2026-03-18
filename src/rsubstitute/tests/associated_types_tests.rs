#![feature(associated_type_defaults)]

use rsubstitute::prelude::*;
use std::fmt::Debug;

#[mock]
trait Trait {
    const MY_CONST: usize = 43;

    type InputType<TAmogus: Clone>: Clone + Debug
        = i32
    where
        Self: Clone;

    type ReturnType<TT>: Clone + Sized
        = u8
    where
        Self: Sized,
        TT: Clone;

    fn get_const(&self) -> usize {
        Self::MY_CONST
    }

    fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::ReturnType<TT>
    where
        Self: Clone + Sized,
        TT: ToString;
}

mocked! {
    #[derive(Clone)]
    struct Struct;
    
    impl Struct {
        pub fn new() -> Self {
            Self
        }
    }
    
    impl Trait for Struct {
        fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::ReturnType<TT>
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[test]
    fn compile() {
        // let mock = TraitMock::<3, i32>::new();
    }
}
