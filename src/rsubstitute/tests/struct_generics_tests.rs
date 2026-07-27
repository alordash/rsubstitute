use rsubstitute::*;
use std::fmt::Debug;

// TODO - move source mock target from inside generated module and place it before generated module
trait Trait {}

#[mock]
pub struct Struct<'a, T1: ToString, T2>
where
    T2: AsRef<[i32]>,
{
    pub t1: T1,
    pub t2_ref: &'a T2,
    pub number: i32,
}

#[mock(base)]
impl<'a, T1: ToString, T2: AsRef<[i32]>> Trait for Struct<'a, T1, T2> {}

#[mock(base)]
impl<'a, T1: Debug + ToString, T2: Debug + AsRef<[i32]>> Struct<'a, T1, T2> {
    pub fn flex(&self) {}

    pub fn get_t2(&self) -> &'a T2 {
        self.t2_ref
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
        let mut mock = Struct {
            t1,
            t2_ref: &t2,
            number,
        }
        .mock();

        let another_t2 = vec![11, 2];
        mock.setup().get_t2().returns(&another_t2);

        let actual_t2 = mock.get_t2();

        assert_ne!(&t2, actual_t2);
    }
}
