pub(crate) trait IMutRefClone {
    fn mut_ref_clone(&self) -> Self;
}

// Required to allow mocking functions that return `&mut T`.
// Default `Clone` is not implemented because only one mutable reference can exist.
// Breaking this rule because return value must be cloneable in order for mock
// to return the same value multiple times.
impl<T> IMutRefClone for &mut T {
    fn mut_ref_clone(&self) -> Self {
        unsafe { std::mem::transmute_copy(&self) }
    }
}
