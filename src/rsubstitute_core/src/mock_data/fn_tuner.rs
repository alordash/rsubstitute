use crate::fn_parameters::*;
use crate::mock_data::{FnConfig, FnReturnCallbackTuner, FnReturnTuner};
use std::cell::RefCell;
use std::sync::Arc;

pub struct FnTuner<
    'rs,
    TOwner,
    TArgRefsTuple: Copy,
    TReturnValue,
    const SUPPORTS_BASE_CALLING: bool,
> {
    fn_config: Arc<RefCell<FnConfig<'rs>>>,
    owner: &'rs TOwner,
    fn_return_callback_tuner: FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple>,
    pub returns: FnReturnTuner<'rs, TOwner, TArgRefsTuple, TReturnValue>,
}

impl<'rs, TOwner, TArgRefsTuple: Copy, TReturnValue, const SUPPORTS_BASE_CALLING: bool>
    FnTuner<'rs, TOwner, TArgRefsTuple, TReturnValue, SUPPORTS_BASE_CALLING>
{
    pub fn new(fn_config: Arc<RefCell<FnConfig<'rs>>>, owner: &'rs TOwner) -> Self {
        Self {
            fn_config: fn_config.clone(),
            owner,
            fn_return_callback_tuner: FnReturnCallbackTuner::new(fn_config.clone(), owner),
            returns: FnReturnTuner::new(fn_config, owner),
        }
    }

    pub fn returns<'a>(
        &self,
        return_value: TReturnValue,
    ) -> &FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let dyn_return_value = unsafe { core::mem::transmute(DynReturnValue::new(return_value)) };
        let return_value_source = ReturnValueSource::SingleTime(dyn_return_value);
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_return_callback_tuner;
    }
}

impl<'rs, TOwner, TArgRefsTuple: Copy, const SUPPORTS_BASE_CALLING: bool>
    FnTuner<'rs, TOwner, TArgRefsTuple, (), SUPPORTS_BASE_CALLING>
{
    pub fn does(&self, callback: impl FnMut(&TArgRefsTuple) + 'static) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'rs, TOwner, TArgRefsTuple: Copy, TReturnValue>
    FnTuner<'rs, TOwner, TArgRefsTuple, TReturnValue, true>
{
    pub fn call_base(&self) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_call_base();
        return self.owner;
    }
}
