use crate::fn_parameters::*;
use crate::mock_data::{FnConfig, FnReturnCallbackTuner, FnReturnTuner};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct FnTuner<'rs, TOwner, TReturnValue> {
    _phantom_return_value: PhantomData<TReturnValue>,
    fn_config: Arc<RefCell<FnConfig<'rs>>>,
    owner: &'rs TOwner,
    fn_return_callback_tuner: FnReturnCallbackTuner<'rs, TOwner>,
    pub returns: FnReturnTuner<'rs, TOwner, TReturnValue>,
}

impl<'rs, TOwner, TReturnValue> FnTuner<'rs, TOwner, TReturnValue> {
    pub fn new(fn_config: Arc<RefCell<FnConfig<'rs>>>, owner: &'rs TOwner) -> Self {
        Self {
            _phantom_return_value: PhantomData,
            fn_config: fn_config.clone(),
            owner,
            fn_return_callback_tuner: FnReturnCallbackTuner::new(fn_config.clone(), owner),
            returns: FnReturnTuner::new(fn_config, owner),
        }
    }

    pub fn returns<'a>(&self, return_value: TReturnValue) -> &FnReturnCallbackTuner<'rs, TOwner>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let dyn_return_value = unsafe { std::mem::transmute(DynReturnValue::new(return_value)) };
        let return_value_source = ReturnValueSource::SingleTime(dyn_return_value);
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_return_callback_tuner;
    }
}

impl<'rs, TOwner> FnTuner<'rs, TOwner, ()> {
    pub fn does(&self, callback: impl FnMut() + 'static) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn does_nothing(&self) -> &'rs TOwner {
        return self.owner;
    }
}

// TODO - support
// impl<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker>
//     FnTuner<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker>
// where
//     TMock: IBaseCaller<TCall, TReturnType>,
// {
//     pub fn call_base(&self) -> &'a TOwner {
//         self.fn_config.borrow_mut().set_call_base();
//         return self.owner;
//     }
// }
