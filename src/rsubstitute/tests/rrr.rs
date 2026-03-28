use rsubstitute_proc_macro::mocked_base;
use std::marker::PhantomData;

trait Trait {
    fn work(&self);
}

mocked_base! {
    struct Struct<T>(PhantomData<T>);
    
    impl Struct<i32> {
        pub fn new() -> Self {
            Self { 0: PhantomData }
        }
    }
}

#[test]
fn compile() {
    let mock = Struct::<u8>::new();
}