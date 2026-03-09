use crate::fn_parameters::*;
use crate::mock_data::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct FnTuner<
    'rs,
    TOwner,
    TArgRefsTuple: Copy,
    TReturnValue,
    const SUPPORTS_BASE_CALLING: bool,
> {
    _phantom_return_value: PhantomData<TReturnValue>,
    fn_config: Arc<RefCell<FnConfig<'rs>>>,
    owner: &'rs TOwner,
    fn_callback_tuner: FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple>,
}

impl<'rs, TOwner, TArgRefsTuple: Copy, TReturnValue, const SUPPORTS_BASE_CALLING: bool>
    FnTuner<'rs, TOwner, TArgRefsTuple, TReturnValue, SUPPORTS_BASE_CALLING>
{
    pub fn new(fn_config: Arc<RefCell<FnConfig<'rs>>>, owner: &'rs TOwner) -> Self {
        Self {
            _phantom_return_value: PhantomData,
            fn_config: fn_config.clone(),
            owner,
            fn_callback_tuner: FnReturnCallbackTuner::new(fn_config.clone(), owner),
        }
    }

    pub fn returns<'a>(
        &self,
        return_value: TReturnValue,
    ) -> &FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let dyn_return_value = unsafe { core::mem::transmute(DynReturnValue::new(return_value)) };
        let return_value_source = ReturnValueSource::SingleTime(dyn_return_value);
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_tuner;
    }

    pub fn returns_many<'a>(
        &self,
        return_values: impl IntoIterator<Item = TReturnValue>,
    ) -> &FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let return_value_sources = return_values
            .into_iter()
            .map(|x| unsafe { core::mem::transmute(DynReturnValue::new(x)) })
            .map(ReturnValueSource::SingleTime);
        self.fn_config
            .borrow_mut()
            .add_return_value_sources(return_value_sources);
        return &self.fn_callback_tuner;
    }

    pub fn returns_always<'a>(
        &self,
        return_value: TReturnValue,
    ) -> &FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple>
    where
        TReturnValue: 'rs + 'a + IReturnValue<'a> + Clone,
    {
        let return_value_source = ReturnValueSource::Perpetual(Box::new(move || unsafe {
            core::mem::transmute(DynReturnValue::new(return_value.clone()))
        }));
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_tuner;
    }

    pub fn returns_with<'a>(
        &self,
        f: impl Fn(TArgRefsTuple) -> TReturnValue + 'rs,
    ) -> &FnReturnCallbackTuner<'rs, TOwner, TArgRefsTuple> {
        let return_value_source = ReturnValueSource::Factory(Box::new(
            move |dyn_arg_refs_tuple: DynArgRefsTuple<'rs>| {
                let arg_refs_tuple: TArgRefsTuple = dyn_arg_refs_tuple.downcast_into();
                let result = f(arg_refs_tuple);
                return unsafe { core::mem::transmute(DynReturnValue::new(result)) };
            },
        ));
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_tuner;
    }
}

impl<'rs, TOwner, TArgRefsTuple: Copy, const SUPPORTS_BASE_CALLING: bool>
    FnTuner<'rs, TOwner, TArgRefsTuple, (), SUPPORTS_BASE_CALLING>
{
    pub fn does(&self, callback: impl FnMut(TArgRefsTuple) + 'static) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'rs, TOwner, TArgRefsTuple: Copy, TReturnValue>
    FnTuner<'rs, TOwner, TArgRefsTuple, TReturnValue, true>
{
    pub fn call_base(&self) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_call_base();
        return self.owner;
    }
}
