use crate::args_matching::IArgsChecker;
use crate::{FnConfig, ICallBase};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct SharedFnConfig<'a, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue, TOwner, TCallBase>
{
    _phantom_base_caller: PhantomData<TCallBase>,
    shared_fn_config: Arc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue, TCallBase>>>,
    owner: &'a TOwner,
}

impl<'a, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue: Clone, TOwner, TCallBase>
    SharedFnConfig<'a, TCall, TArgsChecker, TReturnValue, TOwner, TCallBase>
{
    pub fn new(
        shared_fn_config: Arc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue, TCallBase>>>,
        owner: &'a TOwner,
    ) -> Self {
        Self {
            _phantom_base_caller: PhantomData,
            shared_fn_config,
            owner,
        }
    }

    pub fn returns(&self, return_value: TReturnValue) -> &'a TOwner {
        self.shared_fn_config
            .borrow_mut()
            .add_return_value(return_value);
        return self.owner;
    }

    pub fn returns_many(&self, return_values: &[TReturnValue]) -> &'a TOwner {
        self.shared_fn_config
            .borrow_mut()
            .add_return_values(return_values);
        return self.owner;
    }

    pub fn does(&self, callback: impl FnMut() + 'static) -> &'a TOwner {
        self.shared_fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn returns_and_does(
        &self,
        return_value: TReturnValue,
        callback: impl FnMut() + 'static,
    ) -> &'a TOwner {
        self.returns(return_value);
        self.does(callback);
        return self.owner;
    }
}

impl<
    'a,
    TCall,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue,
    TOwner,
    TCallBase: ICallBase<TCall, TReturnValue>,
> SharedFnConfig<'a, TCall, TArgsChecker, TReturnValue, TOwner, TCallBase>
{
    pub fn call_base(&self) -> &'a TOwner {
        let call_base = TCallBase::new();
        self.shared_fn_config.borrow_mut().set_call_base(call_base);
        return self.owner;
    }
}
