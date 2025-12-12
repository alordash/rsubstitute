use std::cell::RefCell;
use std::rc::Rc;

pub struct FnConfig<TArgsMatcher, TReturnValue> {
    args_matcher: TArgsMatcher,
    return_value: Option<TReturnValue>,
}

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

impl<TArgsMatcher, TReturnValue> FnConfig<TArgsMatcher, TReturnValue> {
    pub fn new(args_matcher: TArgsMatcher) -> Self {
        FnConfig {
            args_matcher,
            return_value: None,
        }
    }

    pub fn set_return_value(&mut self, return_value: TReturnValue) {
        self.return_value = Some(return_value);
    }
}

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

pub mod argument_matching;
