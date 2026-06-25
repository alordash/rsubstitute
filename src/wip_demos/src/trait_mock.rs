trait Trait<T1> {
    fn f<T2>(&self) -> T1;
    fn g<T3>();
}

mod result {
    use Trait_mock::*;
    mod Trait_mock {
        use super::*;
        
        pub trait Trait<T1> {
            fn f<T2>(&self) -> T1;
            fn g<T3>();
        }
    }
}
