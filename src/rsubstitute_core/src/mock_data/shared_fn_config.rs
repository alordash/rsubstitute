use crate::IBaseCaller;
use crate::fn_parameters::*;
use crate::mock_data::FnConfig;
use std::cell::RefCell;
use std::sync::Arc;

// TODO - rename to something like ReturnConfig to better reflect intended usage
pub struct SharedFnConfig<'rs, TOwner, TMock> {
    fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>,
    owner: &'rs TOwner,
}

impl<'rs, TOwner, TMock> SharedFnConfig<'rs, TOwner, TMock> {
    pub fn new(shared_fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>, owner: &'rs TOwner) -> Self {
        Self {
            fn_config: shared_fn_config,
            owner,
        }
    }

    pub fn returns<TReturnValue: IReturnValue<'rs> + 'rs>(
        &self,
        return_value: TReturnValue,
    ) -> &'rs TOwner {
        let dyn_return_value = DynReturnValue::new(return_value);
        self.fn_config
            .borrow_mut()
            .add_return_value(dyn_return_value);
        return self.owner;
    }

    pub fn returns_many<TReturnValue: IReturnValue<'rs> + 'rs, const N: usize>(
        &self,
        return_values: [TReturnValue; N],
    ) -> &'rs TOwner {
        let dyn_return_values = return_values.map(DynReturnValue::new);
        self.fn_config.borrow_mut().add_return_values(dyn_return_values);
        return self.owner;
    }

    pub fn returns_and_does<TReturnValue: IReturnValue<'rs> + 'rs>(
        &self,
        return_value: TReturnValue,
        callback: impl FnMut() + 'static,
    ) -> &'rs TOwner {
        self.returns(return_value);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn returns_many_and_does<TReturnValue: IReturnValue<'rs> + 'rs, const N: usize>(
        &self,
        return_values: [TReturnValue; N],
        callback: impl FnMut() + 'static,
    ) -> &'rs TOwner {
        self.returns_many(return_values);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

// TODO - support
// impl<'rs, TOwner, TMock>
//     SharedFnConfig<'rs, TOwner, TMock>
// {
//     pub fn does(&self, callback: impl FnMut() + 'static) -> &'rs TOwner {
//         self.fn_config.borrow_mut().set_callback(callback);
//         return self.owner;
//     }
// 
//     pub fn does_nothing(&self) -> &'rs TOwner {
//         return self.owner;
//     }
// }

// TODO - support
// impl<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker>
//     SharedFnConfig<'a, TOwner, TMock, TCall, TReturnType, TArgsChecker>
// where
//     TMock: IBaseCaller<TCall, TReturnType>,
// {
//     pub fn call_base(&self) -> &'a TOwner {
//         self.fn_config.borrow_mut().set_call_base();
//         return self.owner;
//     }
// }
