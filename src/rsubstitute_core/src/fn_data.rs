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

    pub fn handle(&self, call: TCall) -> Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>> {
        self.register_call(call.clone());
        let fn_config = self.take_matching_config(call);
        if let Some(callback) = fn_config.borrow().get_callback() {
            callback();
        }
        return fn_config;
    }

    pub fn handle_returning(&self, call: TCall) -> TReturnValue {
        let fn_config = self.handle(call);
        let return_value = fn_config
            .borrow_mut()
            .take_return_value()
            .expect("No return value configured for 'another_work'! TODO: write call description?");
        return return_value;
    }

    fn take_matching_config(
        &self,
        call: TCall,
    ) -> Rc<RefCell<FnConfig<TCall, TArgsMatcher, TReturnValue>>> {
        let index = self
            .configs
            .borrow()
            .iter()
            .position(|config| config.borrow().matches(call.clone()))
            .expect("No fn configuration found for this call! TODO: write call description");
        let fn_config = self.configs.borrow_mut().remove(index);
        return fn_config;
    }
}
