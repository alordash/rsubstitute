use rsubstitute::macros::mock;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[mock]
trait Trait {
    fn mutate(&mut self);

    fn consume(self) -> i32;

    fn sbox(self: Box<Self>) -> i32;
    fn src(self: Rc<Self>) -> i32;
    fn sarc(self: Arc<Self>) -> i32;

    fn spbox(self: Pin<Box<Self>>) -> i32;
    fn sprc(self: Pin<Rc<Self>>) -> i32;
    fn sparc(self: Pin<Arc<Self>>) -> i32;
}

trait Q {
    fn q(self: Box<Self>);
    fn w(self: Rc<Self>);
    fn e(self: Arc<Self>);
    fn qp(self: Pin<Box<Self>>);
    fn wp(self: Pin<Rc<Self>>);
    fn ep(self: Pin<Arc<Self>>);
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
        let mut mock = TraitMock::new();

        mock.setup().sbox().returns(12);
        mock.setup().src().returns(12);
        mock.setup().sarc().returns(12);

        mock.setup().spbox().returns(12);
        mock.setup().sprc().returns(12);
        mock.setup().sparc().returns(12);

        assert_eq!(12, Box::new(mock.clone()).sbox());
        assert_eq!(12, Box::new(mock.clone()).sbox());
        assert_eq!(12, Box::new(mock.clone()).sbox());
        mock.received().sbox(Times::Exactly(3));

        assert_eq!(12, Box::new(mock.clone()).sbox());
        assert_eq!(12, Rc::new(mock.clone()).src());
        assert_eq!(12, Arc::new(mock.clone()).sarc());
        mock.received().sbox(Times::Exactly(4));
        mock.received().src(Times::Once);
        mock.received().sarc(Times::Once);

        assert_eq!(12, Pin::new(Box::new(mock.clone())).spbox());
        assert_eq!(12, Pin::new(Rc::new(mock.clone())).sprc());
        assert_eq!(12, Pin::new(Arc::new(mock.clone())).sparc());
        mock.received().spbox(Times::Once);
        mock.received().sprc(Times::Once);
        mock.received().sparc(Times::Once);

        let flag = Rc::new(Cell::new(false));
        let flag_clone = flag.clone();

        mock.setup().mutate().does(move || flag_clone.set(true));
        mock.setup().consume().returns(22);

        mock.mutate();
        assert!(flag.get());
        let value = mock.clone().consume();
        assert_eq!(22, value);

        mock.received()
            .mutate(Times::Once)
            .consume(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn compile() {}
}
