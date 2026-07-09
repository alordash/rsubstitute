use std::ops::Deref;

pub(crate) struct DerefInfo {
    pub expected_value_deref_ptr: *const (),
    pub deref_vtable_ptr: *const (),
}

// TODO - add `// SAFETY:` comments to unsafe blocks
impl DerefInfo {
    pub fn new<T: Deref<Target = U>, U>(expected_value: &T) -> Self {
        let expected_value_deref_ptr = expected_value.deref() as *const _ as *const ();
        let dyn_ref: &dyn Deref<Target = U> = expected_value;
        let fat_ptr = &dyn_ref as *const _ as *const *const ();
        let deref_vtable_ptr = unsafe { *fat_ptr.add(1) };
        let result = Self {
            expected_value_deref_ptr,
            deref_vtable_ptr,
        };
        return result;
    }

    pub fn get_actual_value_deref_ptr<T>(&self, actual_value: &T) -> *const () {
        let actual_value_ptr = actual_value as *const _ as *const ();
        let raw_fat_pointer = (actual_value_ptr, self.deref_vtable_ptr);
        let actual_value_as_dyn_ref_ref: &dyn Deref<Target = ()> =
            unsafe { core::mem::transmute(raw_fat_pointer) };
        let result = actual_value_as_dyn_ref_ref.deref() as *const ();
        return result;
    }
}
