use crate::infrastructure::*;
use crate::transmute_lifetime;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub trait IMockable<TMock>: Sized {
    fn mock_from_ref(&mut self) -> TMock;

    fn mock<'a>(self) -> TMock {
        let mock_box = Box::new(self);
        // LEAK: mock is leaked because it will be cleared by
        // `TMock`'s `Drop` implementation using `IMock::drop_boxed_mockable`.
        let mock_leaked = Box::leak(mock_box);
        let mock = mock_leaked.mock_from_ref();
        return mock;
    }
}

// Usage
struct Struct<'b> {
    pub data: Arc<Vec<i32>>,
    pub r: &'b [u8; 3],
}

struct StructMock<'a, 'b> {
    mockable: &'a mut Struct<'b>,
}

impl<'a, 'b> IMockable<StructMock<'a, 'b>> for Struct<'b> {
    fn mock_from_ref(&mut self) -> StructMock<'a, 'b> {
        StructMock {
            mockable: transmute_lifetime!(self),
        }
    }
}

impl<'a, 'b> Deref for StructMock<'a, 'b> {
    type Target = Struct<'b>;

    fn deref(&self) -> &Self::Target {
        self.mockable
    }
}

impl<'a, 'b> DerefMut for StructMock<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mockable
    }
}

impl<'a, 'b> IMock<Struct<'b>> for StructMock<'a, 'b> {}

impl<'a, 'b> Drop for StructMock<'a, 'b> {
    fn drop(&mut self) {
        self.drop_boxed_mockable()
    }
}

fn usage(rv: &[u8; 3]) {
    let s = Struct {
        data: Arc::new(vec![5, 3, 2]),
        r: rv,
    };
    let mockable = s.mock();
    let data = &mockable.data;
    let r = &mockable.r;
    dbg!(data, r);
    drop(mockable);
    // ERROR: owner dropped
    // dbg!(data);
    // dbg!(r);
}
