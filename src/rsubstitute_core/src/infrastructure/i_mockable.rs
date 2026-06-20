use crate::infrastructure::*;
use crate::transmute_lifetime;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub trait IMockable<TMock>: Sized {
    fn mock_from_ref(&mut self) -> TMock;

    fn mock<'__rs>(self) -> TMock {
        let mock_box = Box::new(self);
        // LEAK: mock is leaked because it will be cleared by
        // `TMock`'s `Drop` implementation using `IMock::drop_boxed_mockable`.
        let mock_leaked = Box::leak(mock_box);
        let mock = mock_leaked.mock_from_ref();
        return mock;
    }
}

// TODO - remove
// Usage
struct Struct<'b> {
    pub data: Arc<Vec<i32>>,
    pub r: &'b [u8; 3],
}

struct StructMock<'__rs, 'b> {
    mockable: &'__rs mut Struct<'b>,
}

// gen
impl<'__rs, 'b> IMockable<StructMock<'__rs, 'b>> for Struct<'b> {
    fn mock_from_ref(&mut self) -> StructMock<'__rs, 'b> {
        StructMock {
            mockable: transmute_lifetime!(self),
        }
    }
}

// gen
impl<'__rs, 'b> Deref for StructMock<'__rs, 'b> {
    type Target = Struct<'b>;

    fn deref(&self) -> &Self::Target {
        self.mockable
    }
}

// gen
impl<'__rs, 'b> DerefMut for StructMock<'__rs, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mockable
    }
}

// gen
impl<'__rs, 'b> IMock<Struct<'b>> for StructMock<'__rs, 'b> {}

// gen
impl<'__rs, 'b> Drop for StructMock<'__rs, 'b> {
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
