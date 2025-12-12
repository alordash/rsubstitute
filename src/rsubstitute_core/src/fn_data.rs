use crate::FnConfig;
use crate::arguments_matching::IArgsMatcher;
use std::cell::RefCell;
use std::rc::Rc;

pub struct FnData<TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue> {
    calls: RefCell<Vec<TCall>>,
    configs: RefCell<Vec<Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>>>>,
}

impl<TCall, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue> Default
    for FnData<TCall, TArgsMatcher, TReturnValue>
{
    fn default() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            configs: RefCell::new(Vec::new()),
        }
    }
}

impl<TCall: Clone, TArgsMatcher: IArgsMatcher<TCall>, TReturnValue>
    FnData<TCall, TArgsMatcher, TReturnValue>
{
    pub fn register_call(&self, call: TCall) -> &Self {
        self.calls.borrow_mut().push(call);
        self
    }

    pub fn add_config(
        &self,
        args_matcher: TArgsMatcher,
    ) -> Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>> {
        let config = FnConfig::new(args_matcher);
        let shared_config = Rc::new(RefCell::new(config));
        self.configs.borrow_mut().push(shared_config.clone());
        return shared_config;
    }

    pub fn get_matching_config(
        &self,
        call: TCall,
    ) -> Option<Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>>> {
        self.configs
            .borrow()
            .iter()
            .find(|config| config.borrow().matches(call.clone()))
            .cloned()
    }
}
