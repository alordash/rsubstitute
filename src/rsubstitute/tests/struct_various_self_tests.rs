use rsubstitute::macros::*;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

mocked! {
    #[derive(Clone)]
    struct Struct;

    impl Struct {
        pub fn new() -> Self { Self }

        pub fn mutate(&mut self) {}

        pub fn consume(self) -> i32 { 10 }


        pub fn sbox(self: Box<Self>) -> i32 { 212 }
        pub fn src(self: Rc<Self>) -> i32 { 212 }
        pub fn sarc(self: Arc<Self>) -> i32 { 212 }

        pub fn spbox(self: Pin<Box<Self>>) -> i32 { 212 }
        pub fn sprc(self: Pin<Rc<Self>>) -> i32 { 212 }
        pub fn sparc(self: Pin<Arc<Self>>) -> i32 { 212 }
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

        mock.setup().sbox().call_base();
        mock.setup().src().call_base();
        mock.setup().sarc().call_base();

        mock.setup().spbox().call_base();
        mock.setup().sprc().call_base();
        mock.setup().sparc().call_base();

        assert_eq!(212, Box::new(mock.clone()).sbox());
        assert_eq!(212, Box::new(mock.clone()).sbox());
        assert_eq!(212, Box::new(mock.clone()).sbox());
        mock.received().sbox(Times::Exactly(3));

        assert_eq!(212, Box::new(mock.clone()).sbox());
        assert_eq!(212, Rc::new(mock.clone()).src());
        assert_eq!(212, Arc::new(mock.clone()).sarc());
        mock.received().sbox(Times::Exactly(4));
        mock.received().src(Times::Once);
        mock.received().sarc(Times::Once);

        assert_eq!(212, Pin::new(Box::new(mock.clone())).spbox());
        assert_eq!(212, Pin::new(Rc::new(mock.clone())).sprc());
        assert_eq!(212, Pin::new(Arc::new(mock.clone())).sparc());
        mock.received().spbox(Times::Once);
        mock.received().sprc(Times::Once);
        mock.received().sparc(Times::Once);

        let flag = Rc::new(Cell::new(false));
        let flag_clone = flag.clone();

        mock.setup().mutate().does(move || flag_clone.set(true));
        mock.setup().consume().call_base();

        mock.mutate();
        assert!(flag.get());
        let value = mock.clone().consume();
        assert_eq!(10, value);

        mock.received()
            .mutate(Times::Once)
            .consume(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn compile() {}
}
