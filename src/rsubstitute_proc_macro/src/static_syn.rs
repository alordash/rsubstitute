use std::ops::Deref;
use std::sync::LazyLock;

pub struct StaticSyn<T> {
    lazy: LazyLock<T>,
}

impl<T> StaticSyn<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            lazy: LazyLock::new(init),
        }
    }
}

unsafe impl<T> Send for StaticSyn<T> {}
unsafe impl<T> Sync for StaticSyn<T> {}

impl<T> Deref for StaticSyn<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.lazy
    }
}
