use crate::mock_data::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

pub struct FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple: Copy> {
    _phantom_args_tuple: PhantomData<TArgRefsTuple>,
    fn_config: Arc<RefCell<FnConfig<'rs>>>,
    owner: &'rs TOwner,
}

impl<'rs, TOwner, TArgRefsTuple: Copy> FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple> {
    pub(crate) fn new(fn_config: Arc<RefCell<FnConfig<'rs>>>, owner: &'rs TOwner) -> Self {
        Self {
            _phantom_args_tuple: PhantomData,
            fn_config,
            owner,
        }
    }

    pub fn and_does<'a>(&self, callback: impl FnMut(TArgRefsTuple) + 'static) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'rs, TOwner, TArgRefsTuple: Copy> Deref for FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple> {
    type Target = TOwner;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}
