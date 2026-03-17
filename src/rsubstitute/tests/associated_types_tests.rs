#![feature(associated_type_defaults)]

use rsubstitute::prelude::*;

#[mock]
trait Trait {
    const MY_CONST: usize;

    type MyType<TT>: Clone
    where
        Self: Sized,
        TT: Clone;
}

mocked! {
    struct Struct;

    impl Struct {
        pub fn new() -> Self {
            Self
        }
    }

    impl Trait for Struct {
        const MY_CONST: usize = 643;
        type MyType<TT>
        = Vec<TT>
    where
        Self: Sized,
        TT: Clone;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[test]
    fn compile() {}
}
