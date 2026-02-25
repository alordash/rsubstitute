use crate::IBaseCaller;
use crate::fn_parameters::*;
use crate::mock_data::FnConfig;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

// TODO - rename to something like ReturnConfig to better reflect intended usage
pub struct SharedFnConfig<'rs, TOwner, TMock, TReturnValue> {
    _phantom_return_value: PhantomData<TReturnValue>,
    fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>,
    owner: &'rs TOwner,
}

impl<'rs, TOwner, TMock, TReturnValue> SharedFnConfig<'rs, TOwner, TMock, TReturnValue> {
    pub fn new(shared_fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>, owner: &'rs TOwner) -> Self {
        Self {
            _phantom_return_value: PhantomData,
            fn_config: shared_fn_config,
            owner,
        }
    }

    pub fn returns<'a>(&self, return_value: TReturnValue) -> &'rs TOwner
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let dyn_return_value = unsafe { std::mem::transmute(DynReturnValue::new(return_value)) };
        let return_value_source = ReturnValueSource::SingleTime(dyn_return_value);
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return self.owner;
    }

    pub fn returns_many<'a>(
        &self,
        return_values: impl IntoIterator<Item = TReturnValue>,
    ) -> &'rs TOwner
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let return_value_sources = return_values
            .into_iter()
            .map(|x| unsafe { std::mem::transmute(DynReturnValue::new(x)) })
            .map(ReturnValueSource::SingleTime);
        self.fn_config
            .borrow_mut()
            .add_return_value_sources(return_value_sources);
        return self.owner;
    }

    pub fn returns_always<'a>(&self, return_value: TReturnValue) -> &'rs TOwner
    where
        TReturnValue: 'rs + 'a + IReturnValue<'a> + Clone,
    {
        let return_value_source = ReturnValueSource::Perpetual(Box::new(move || unsafe {
            std::mem::transmute(DynReturnValue::new(return_value.clone()))
        }));
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return self.owner;
    }

    pub fn returns_and_does<'a>(
        &self,
        return_value: TReturnValue,
        callback: impl FnMut() + 'static,
    ) -> &'rs TOwner
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        self.returns(return_value);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn returns_many_and_does<'a>(
        &self,
        return_values: impl IntoIterator<Item = TReturnValue>,
        callback: impl FnMut() + 'static,
    ) -> &'rs TOwner
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        self.returns_many(return_values);
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }

    pub fn returns_always_and_does<'a>(
        &self,
        return_value: TReturnValue,
        callback: impl FnMut() + 'static,
    ) -> &'rs TOwner
    where
        TReturnValue: 'rs + 'a + IReturnValue<'a> + Clone,
    {
        self.returns_always(return_value);
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
