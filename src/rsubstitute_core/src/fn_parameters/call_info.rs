use crate::fn_parameters::Call;

pub(crate) struct CallInfo<'a> {
    verified: bool,
    call: Call<'a>,
}

impl<'a> CallInfo<'a> {
    pub fn new(call: Call<'a>) -> Self {
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

    pub fn get_call(&self) -> &Call<'a> {
        &self.call
    }
}
