use crate::{FnConfig, IBaseCaller};
use std::cell::RefCell;
use std::sync::Arc;

// TODO - rename to something like ReturnConfig to better reflect intended usage
pub struct SharedFnConfig<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker> {
    fn_config: Arc<RefCell<FnConfig<TMock, TCall, TReturnType, TArgsChecker>>>,
    owner: &'a TOwner,
}

impl<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker>
    SharedFnConfig<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker>
{
    pub fn new(
        shared_fn_config: Arc<RefCell<FnConfig<TMock, TCall, TReturnType, TArgsChecker>>>,
        owner: &'a TOwner,
    ) -> Self {
        Self {
            fn_config: shared_fn_config,
            owner,
        }
    }

    pub fn returns(&self, return_value: TReturnType) -> &'a TOwner {
        self.fn_config.borrow_mut().add_return_value(return_value);
        return self.owner;
    }

    pub fn returns_many<const N: usize>(&self, return_values: [TReturnType; N]) -> &'a TOwner {
        self.fn_config.borrow_mut().add_return_values(return_values);
        return self.owner;
    }

    pub fn returns_and_does(
        &self,
        return_value: TReturnType,
        callback: impl FnMut() + 'static,
    ) -> &'a TOwner {
        self.returns(return_value);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn returns_many_and_does<const N: usize>(
        &self,
        return_values: [TReturnType; N],
        callback: impl FnMut() + 'static,
    ) -> &'a TOwner {
        self.returns_many(return_values);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'a, TOwner, TMock, TCall, TArgsChecker>
    SharedFnConfig<'a, TOwner, TMock, TCall, (), TArgsChecker>
{
    pub fn does(&self, callback: impl FnMut() + 'static) -> &'a TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn does_nothing(&self) -> &'a TOwner {
        return self.owner;
    }
}

impl<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker>
    SharedFnConfig<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker>
where
    TMock: IBaseCaller<TCall, TReturnType>,
{
    pub fn call_base(&self) -> &'a TOwner {
        self.fn_config.borrow_mut().set_call_base();
        return self.owner;
    }
}
