use crate::args_matching::IArgsChecker;
use crate::{FnConfig, IBaseCaller};
use std::cell::RefCell;
use std::sync::Arc;

pub struct SharedFnConfig<'a, TMock, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue, TOwner>
{
    fn_config: Arc<RefCell<FnConfig<TMock, TCall, TArgsChecker, TReturnValue>>>,
    owner: &'a TOwner,
}

impl<'a, TMock, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue: Clone, TOwner>
    SharedFnConfig<'a, TMock, TCall, TArgsChecker, TReturnValue, TOwner>
{
    pub fn new(
        shared_fn_config: Arc<RefCell<FnConfig<TMock, TCall, TArgsChecker, TReturnValue>>>,
        owner: &'a TOwner,
    ) -> Self {
        Self {
            fn_config: shared_fn_config,
            owner,
        }
    }

    pub fn returns(&self, return_value: TReturnValue) -> &'a TOwner {
        self.fn_config.borrow_mut().add_return_value(return_value);
        return self.owner;
    }

    pub fn returns_many(&self, return_values: &[TReturnValue]) -> &'a TOwner {
        self.fn_config.borrow_mut().add_return_values(return_values);
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

    pub fn returns_many_and_does(
        &self,
        return_values: &[TReturnValue],
        callback: impl FnMut() + 'static,
    ) -> &'a TOwner {
        self.returns_many(return_values);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'a, TMock, TCall, TArgsChecker: IArgsChecker<TCall>, TOwner>
    SharedFnConfig<'a, TMock, TCall, TArgsChecker, (), TOwner>
{
    pub fn does(&self, callback: impl FnMut() + 'static) -> &'a TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<
    'a,
    TMock: IBaseCaller<TCall, TReturnValue>,
    TCall,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue,
    TOwner,
> SharedFnConfig<'a, TMock, TCall, TArgsChecker, TReturnValue, TOwner>
{
    pub fn call_base(&self) -> &'a TOwner {
        self.fn_config.borrow_mut().set_call_base();
        return self.owner;
    }
}
