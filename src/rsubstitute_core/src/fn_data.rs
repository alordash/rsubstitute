use crate::FnConfig;
use std::cell::RefCell;
use std::rc::Rc;

pub struct FnData<TCall, TArgsMatcher, TReturnValue> {
    calls: RefCell<Vec<TCall>>,
    configs: RefCell<Vec<Rc<RefCell<FnConfig<TArgsMatcher, TReturnValue>>>>>,
}

impl<TCall, TArgsMatcher, TReturnValue> Default for FnData<TCall, TArgsMatcher, TReturnValue> {
    fn default() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            configs: RefCell::new(Vec::new()),
        }
    }
}

impl<TCall, TArgsMatcher, TReturnValue> FnData<TCall, TArgsMatcher, TReturnValue> {
    pub fn register_call(&self, call: TCall) -> &Self {
        self.calls.borrow_mut().push(call);
        self
    }

    pub fn add_config(
        &self,
        args_matcher: TArgsMatcher,
    ) -> Rc<RefCell<FnConfig<TArgsMatcher, TReturnValue>>> {
        let config = FnConfig::new(args_matcher);
        let shared_config = Rc::new(RefCell::new(config));
        self.configs.borrow_mut().push(shared_config.clone());
        return shared_config;
    }
}
