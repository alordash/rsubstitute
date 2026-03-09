use crate::mock_data::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

pub struct FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, const STORES_MOCK_DATA: bool> {
    _phantom_args_tuple: PhantomData<TArgRefsTuple>,
    fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>,
    owner: &'rs TOwner,
}

impl<'rs, TMock, TOwner, TArgRefsTuple, const STORES_MOCK_DATA: bool>
    FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, STORES_MOCK_DATA>
{
    pub(crate) fn new(fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>, owner: &'rs TOwner) -> Self {
        Self {
            _phantom_args_tuple: PhantomData,
            fn_config,
            owner,
        }
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple>
    FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, false>
{
    pub fn and_does<'a>(&self, mut callback: impl FnMut(TArgRefsTuple) + 'static) -> &'rs TOwner {
        let callback_with_mock =
            move |_mock: &TMock, arg_refs_tuple: TArgRefsTuple| callback(arg_refs_tuple);
        self.fn_config.borrow_mut().set_callback(callback_with_mock);
        return self.owner;
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple>
    FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, true>
{
    pub fn and_does<'a>(
        &self,
        callback: impl FnMut(&TMock, TArgRefsTuple) + 'static,
    ) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple, const STORES_MOCK_DATA: bool> Deref
    for FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, STORES_MOCK_DATA>
{
    type Target = TOwner;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}
