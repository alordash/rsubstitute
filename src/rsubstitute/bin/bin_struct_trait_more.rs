use rsubstitute::prelude::*;
use std::marker::PhantomData;

trait Trait<T3, T4> {
    fn gork(&self);
}

// mocked_base! {
    struct Struct<T1, T2>(PhantomData<(T1, T2)>);

    impl<T1, T2> Struct<T1, T2> {
        pub fn new() -> Self {
            Self::new()
        }
    }

    impl<T1, T4> Trait<i32, T4> for Struct<T1, u8> {
        fn gork(&self) {}
    }
    
    impl<T1, T4> Trait<f32, T4> for Struct<T1, u8> {
        fn gork(&self) {}
    }
// }

fn main() {
    println!("Done");
}
