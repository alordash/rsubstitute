pub(crate) struct CallInfo<TCall> {
    verified: bool,
    call_ptr: *const TCall,
}

impl<TCall> CallInfo<TCall> {
    pub fn new(call: &TCall) -> Self {
        let call_ptr = std::ptr::from_ref(call);
        Self {
            verified: false,
            call_ptr,
        }
    }

    pub fn verify(&mut self) {
        self.verified = true;
    }

    pub fn is_not_verified(&self) -> bool {
        !self.verified
    }

    pub fn get_call(&self) -> &TCall {
        unsafe {
            self.call_ptr
                .as_ref()
                .expect("Call ptr should not be null, calls basically should have static lifetime.")
        }
    }
}
