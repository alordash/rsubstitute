pub(crate) struct AsRefInfo {
    pub value_as_ref_ptr: *const (),
    pub as_ref_vtable_ptr: *const (),
}

// TODO - add `// SAFETY:` comments to unsafe blocks
impl AsRefInfo {
    pub fn new<T: AsRef<U>, U>(value: &T) -> Self {
        let value_as_ref_ptr = value.as_ref() as *const _ as *const ();
        let dyn_ref: &dyn AsRef<U> = value;
        let dyn_ref_ptr = &dyn_ref as *const _ as *const *const ();
        let as_ref_vtable_ptr = unsafe { *dyn_ref_ptr.add(1) };
        let result = Self {
            value_as_ref_ptr,
            as_ref_vtable_ptr,
        };
        return result;
    }

    pub fn get_actual_value_as_ref_ptr<T>(&self, actual_value: &T) -> *const () {
        let actual_value_dyn_ref: &dyn IDynRef = actual_value;
        let actual_value_dyn_ref_ptr = &actual_value_dyn_ref as *const _ as *mut *const ();
        unsafe {
            *actual_value_dyn_ref_ptr.add(1) = self.as_ref_vtable_ptr;
            let v_dyn_as_ref_ref: &dyn AsRef<()> = core::mem::transmute(actual_value_dyn_ref);
            let result = v_dyn_as_ref_ref.as_ref() as *const ();
            return result;
        }
    }
}

trait IDynRef {}
impl<T> IDynRef for T {}
