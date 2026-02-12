use crate::{FnConfig, IBaseCaller, IRawReturnValue, ReturnValue};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct SharedFnConfig<'a, TMock, TArgsChecker, TReturnValue: IRawReturnValue, TOwner> {
    _phantom_return_value: PhantomData<TReturnValue>,
    fn_config: Arc<RefCell<FnConfig<TMock, TArgsChecker>>>,
    owner: &'a TOwner,
}

impl<'a, TMock, TArgsChecker, TReturnValue: IRawReturnValue, TOwner>
    SharedFnConfig<'a, TMock, TArgsChecker, TReturnValue, TOwner>
{
    pub fn new(
        shared_fn_config: Arc<RefCell<FnConfig<TMock, TArgsChecker>>>,
        owner: &'a TOwner,
    ) -> Self {
        Self {
            _phantom_return_value: PhantomData,
            fn_config: shared_fn_config,
            owner,
        }
    }

    pub fn returns(&self, return_value: TReturnValue) -> &'a TOwner {
        self.fn_config
            .borrow_mut()
            .add_return_value(ReturnValue::new(return_value));
        return self.owner;
    }

    pub fn returns_many<const N: usize>(&self, return_values: [TReturnValue; N]) -> &'a TOwner {
        self.fn_config
            .borrow_mut()
            .add_return_values(return_values.map(ReturnValue::new));
        return self.owner;
    }

    pub fn returns_and_does(
        &self,
        return_value: TReturnValue,
        callback: impl FnMut() + 'static,
    ) -> &'a TOwner {
        self.returns(return_value);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn returns_many_and_does<const N: usize>(
        &self,
        return_values: [TReturnValue; N],
        callback: impl FnMut() + 'static,
    ) -> &'a TOwner {
        self.returns_many(return_values);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'a, TMock, TArgsChecker, TOwner> SharedFnConfig<'a, TMock, TArgsChecker, (), TOwner> {
    pub fn does(&self, callback: impl FnMut() + 'static) -> &'a TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'a, TMock: IBaseCaller, TArgsChecker, TReturnValue: IRawReturnValue, TOwner>
    SharedFnConfig<'a, TMock, TArgsChecker, TReturnValue, TOwner>
{
    pub fn call_base(&self) -> &'a TOwner {
        self.fn_config.borrow_mut().set_call_base();
        return self.owner;
    }
}
