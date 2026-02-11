pub trait IMutRefClone {
    fn mut_ref_clone(&self) -> Self;
}

impl<T> IMutRefClone for &mut T {
    fn mut_ref_clone(&self) -> Self {
        unsafe { std::mem::transmute_copy(self) }
    }
}
