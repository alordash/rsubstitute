use crate::FnConfig;
use crate::args_matching::IArgsChecker;
use std::cell::RefCell;
use std::sync::Arc;

pub struct SharedFnConfig<'a, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue, TOwner> {
    shared_fn_config: Arc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue>>>,
    owner: &'a TOwner,
}

impl<'a, TCall, TArgsChecker: IArgsChecker<TCall>, TReturnValue: Clone, TOwner>
    SharedFnConfig<'a, TCall, TArgsChecker, TReturnValue, TOwner>
{
    pub fn new(
        shared_fn_config: Arc<RefCell<FnConfig<TCall, TArgsChecker, TReturnValue>>>,
        owner: &'a TOwner,
    ) -> Self {
        Self {
            shared_fn_config,
            owner,
        }
    }

    pub fn returns(&self, return_value: TReturnValue) -> &'a TOwner {
        self.shared_fn_config
            .borrow_mut()
            .set_return_value(return_value);
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
