use crate::args_matching::IArgsChecker;
use crate::{FnConfig, IBaseCaller};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct SharedFnConfig<
    'a,
    TMock,
    TCall,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue,
    TOwner,
    TBaseCaller,
> {
    _phantom_base_caller: PhantomData<TBaseCaller>,
    fn_config: Arc<RefCell<FnConfig<TMock, TCall, TArgsChecker, TReturnValue, TBaseCaller>>>,
    owner: &'a TOwner,
    base_caller: Option<Arc<RefCell<TBaseCaller>>>,
}

impl<'a, TMock, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue: Clone, TOwner, TBaseCaller>
    SharedFnConfig<'a, TMock, TCall, TArgsChecker, TReturnValue, TOwner, TBaseCaller>
{
    pub fn new(
        shared_fn_config: Arc<RefCell<FnConfig<TMock, TCall, TArgsChecker, TReturnValue, TBaseCaller>>>,
        owner: &'a TOwner,
        base_caller: Option<Arc<RefCell<TBaseCaller>>>,
    ) -> Self {
        Self {
            _phantom_base_caller: PhantomData,
            fn_config: shared_fn_config,
            owner,
            base_caller,
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

impl<'a, TMock, TCall, TArgsChecker: IArgsChecker<TCall>, TOwner, TBaseCaller>
    SharedFnConfig<'a, TMock, TCall, TArgsChecker, (), TOwner, TBaseCaller>
{
    pub fn does(&self, callback: impl FnMut() + 'static) -> &'a TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<
    'a,
    TMock,
    TCall,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnValue,
    TOwner,
    TBaseCaller: IBaseCaller<TMock, TCall, TReturnValue>,
> SharedFnConfig<'a, TMock, TCall, TArgsChecker, TReturnValue, TOwner, TBaseCaller>
{
    pub fn call_base(&self) -> &'a TOwner {
        let base_caller = self
            .base_caller
            .as_ref()
            .expect("Base caller should be set since it implements `IBaseCaller`.")
            .clone();
        self.fn_config.borrow_mut().set_base_caller(base_caller);
        return self.owner;
    }
}
