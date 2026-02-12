use rsubstitute::macros::*;

mocked! {
    struct Struct;

    impl Struct {
        pub fn new() -> Self { Self }

        pub fn mutate(&mut self) {}

        pub fn consume(self) -> i32 { 10 }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn flex() {
        let mut mock = StructMock::new();

        let flag = Rc::new(Cell::new(false));
        let flag_clone = flag.clone();

        mock.setup().mutate().does(move || flag_clone.set(true));
        mock.setup().consume().call_base();

        mock.mutate();
        assert!(flag.get());
        let value = mock.consume();
        assert_eq!(10, value);
    }

    #[test]
    fn compile() {}
}
