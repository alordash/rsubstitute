use crate::FnConfig;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SharedFnConfig<'a, TArgsMatcher, TReturnValue, TOwner> {
    shared_fn_config: Rc<RefCell<FnConfig<TArgsMatcher, TReturnValue>>>,
    owner: &'a TOwner,
}

impl<'a, TArgsMatcher, TReturnValue, TOwner>
    SharedFnConfig<'a, TArgsMatcher, TReturnValue, TOwner>
{
    pub fn new(
        shared_fn_config: Rc<RefCell<FnConfig<TArgsMatcher, TReturnValue>>>,
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
}
