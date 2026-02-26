use crate::mock_data::*;
use std::cell::RefCell;
use std::ops::Deref;
use std::sync::Arc;

pub struct FnReturnCallbackTuner<'rs, TOwner> {
    fn_config: Arc<RefCell<FnConfig<'rs>>>,
    owner: &'rs TOwner,
}

impl<'rs, TOwner> FnReturnCallbackTuner<'rs, TOwner> {
    pub(crate) fn new(fn_config: Arc<RefCell<FnConfig<'rs>>>, owner: &'rs TOwner) -> Self {
        Self { fn_config, owner }
    }

    pub fn and_does<'a>(&self, callback: impl FnMut() + 'static) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'rs, TOwner> Deref for FnReturnCallbackTuner<'rs, TOwner> {
    type Target = TOwner;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}
