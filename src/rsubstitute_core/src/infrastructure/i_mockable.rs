use crate::infrastructure::IMock;
use crate::transmute_lifetime;
use std::sync::Arc;

pub trait IMocked<TMock>: Sized {
    fn mock_from_ref(&mut self) -> TMock;

    fn mock<'a>(self) -> (&'a mut Self, TMock) {
        let mock_box = Box::new(self);
        // LEAK: mock is leaked because it will be cleared by
        // `TMock`'s `Drop` implementation using `IMock::drop_boxed_mocked`.
        let mock_leaked = Box::leak(mock_box);
        let mock = mock_leaked.mock_from_ref();
        return (mock_leaked, mock);
    }
}

struct Struct<'b> {
    pub data: Arc<Vec<i32>>,
    pub r: &'b [u8; 3],
}

struct StructMock<'a, 'b> {
    pub mocked: &'a mut Struct<'b>,
}

impl<'a, 'b> IMocked<StructMock<'a, 'b>> for Struct<'b> {
    fn mock_from_ref(&mut self) -> StructMock<'a, 'b> {
        StructMock {
            mocked: transmute_lifetime!(self),
        }
    }
}

impl<'a, 'b> IMock<Struct<'b>> for StructMock<'a, 'b> {
    fn get_boxed_mocked(&mut self) -> &mut Struct<'b> {
        self.mocked
    }
}

impl<'a, 'b> Drop for StructMock<'a, 'b> {
    fn drop(&mut self) {
        self.drop_boxed_mocked()
    }
}
