use crate::arguments_matching::IArgsMatcher;
use crate::FnConfig;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SharedFnConfig<'a, TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue, TOwner> {
    shared_fn_config: Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>>,
    owner: &'a TOwner,
}

impl<'a, TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue: Clone, TOwner>
    SharedFnConfig<'a, TCall, TArgsMatcher, TReturnValue, TOwner>
{
    pub fn new(
        shared_fn_config: Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>>,
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

    pub fn does(&self, callback: fn()) -> &'a TOwner {
        self.shared_fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn returns_and_does(&self, return_value: TReturnValue, callback: fn()) -> &'a TOwner {
        self.returns(return_value);
        self.does(callback);
        return self.owner;
    }
}
