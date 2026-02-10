use rsubstitute::macros::*;
use std::fmt::Debug;

mocked! {
    struct Struct<'a, T1: Debug, T2>
    where
        T2: ToString,
    {
        t1: T1,
        t2_ref: &'a T2,
        number: i32,
    }

    impl<'a, T1: Debug, T2: ToString> Struct<'a, T1, T2> {
        pub fn new(t1: T1, t2_ref: &'a T2, number: i32) -> Self {
            Self { t1, t2_ref, number }
        }

        pub fn flex(&self) {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compile() {}
}
