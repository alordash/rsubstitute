use rsubstitute::macros::*;
use std::fmt::Debug;

mocked! {
    struct Struct<'a, T1: ToString, T2>
    where
        T2: AsRef<[i32]>,
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

        pub fn get_t2(&self) -> &'a T2 {
            self.t2_ref
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile() {}

    #[test]
    fn test_lifetimes() {
        let t1 = "amogus";
        let t2 = vec![3, 4, 5];
        let number = 4534;
        let mock = StructMock::new(t1, &t2, number);

        let another_t2 = vec![11, 2];
        mock.setup().get_t2().returns(&another_t2);

        let actual_t2 = mock.get_t2();

        assert_ne!(&t2, actual_t2);
    }
}
