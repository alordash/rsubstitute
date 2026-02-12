use crate::fn_parameters::Call;

pub(crate) struct CallInfo {
    verified: bool,
    call: Call,
}

impl CallInfo {
    pub fn new(call: Call) -> Self {
        Self {
            verified: false,
            call,
        }
    }

    pub fn verify(&mut self) {
        self.verified = true;
    }

    pub fn is_not_verified(&self) -> bool {
        !self.verified
    }

    pub fn get_call(&self) -> &Call {
        &self.call
    }
}
