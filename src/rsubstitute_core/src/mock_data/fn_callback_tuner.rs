use crate::mock_data::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

pub struct FnReturnCallbackTuner<
    'rs,
    TMock,
    TOwner,
    TArgRefsTuple,
    TMockArg,
    const STORES_MOCK_DATA: bool,
> {
    _phantom_args_tuple: PhantomData<TArgRefsTuple>,
    _phantom_mock_arg: PhantomData<TMockArg>,
    fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>,
    owner: &'rs TOwner,
}

impl<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, const STORES_MOCK_DATA: bool>
    FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, STORES_MOCK_DATA>
{
    pub(crate) fn new(fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>, owner: &'rs TOwner) -> Self {
        Self {
            _phantom_args_tuple: PhantomData,
            _phantom_mock_arg: PhantomData,
            fn_config,
            owner,
        }
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple, TMockArg>
    FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, false>
{
    pub fn and_does<'a>(&self, mut callback: impl FnMut(TArgRefsTuple) + 'static) -> &'rs TOwner {
        let callback_with_mock =
            move |_mock: &TMock, arg_refs_tuple: TArgRefsTuple| callback(arg_refs_tuple);
        self.fn_config.borrow_mut().set_callback(callback_with_mock);
        return self.owner;
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple, TMockArg>
    FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, true>
{
    pub fn and_does<'a>(
        &self,
        callback: impl FnMut(&TMockArg, TArgRefsTuple) + 'static,
    ) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, const STORES_MOCK_DATA: bool> Deref
    for FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, TMockArg, STORES_MOCK_DATA>
{
    type Target = TOwner;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}
