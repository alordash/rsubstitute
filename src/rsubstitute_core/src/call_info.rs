pub(crate) struct CallInfo<TCall> {
    verified: bool,
    call: TCall,
}

impl<TCall> CallInfo<TCall> {
    pub fn new(call: TCall) -> Self {
        Self {
            verified: false,
            call,
        }
    }

    pub fn verify(&mut self) {
        self.verified = true;
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }

    pub fn get_call(&self) -> &TCall {
        &self.call
    }
}
