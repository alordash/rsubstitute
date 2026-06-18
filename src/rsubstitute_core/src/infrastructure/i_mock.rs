use std::ops::DerefMut;

pub trait IMock<TMockable>: Sized + DerefMut<Target = TMockable> {
    fn drop_boxed_mockable(&mut self) {
        let mockable = self.deref_mut();
        // SAFETY: this frees memory leaked from `IMockable::mock` method.
        unsafe {
            let _ = Box::from_raw(mockable as *mut _);
        }
    }
}

