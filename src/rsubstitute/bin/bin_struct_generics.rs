use std::marker::PhantomData;
use rsubstitute_proc_macro::mocked_base;

// mocked_base! {
    struct Struct<T>(PhantomData<T>);
    
    impl Struct<i32> {
        pub fn new() -> Self {
            Self { 0: PhantomData }
        }
        
        pub fn flex(&self) {}
    }
    
    impl Struct<f32> {
        pub fn flex(&self) {}
    }
// }

fn main() {
    println!("Done");
}