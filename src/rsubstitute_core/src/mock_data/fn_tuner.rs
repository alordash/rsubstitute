use crate::fn_parameters::*;
use crate::mock_data::*;
use crate::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct FnTuner<
    'rs,
    TMock,
    TOwner,
    TArgRefsTuple: Copy,
    TReturnValue,
    const SUPPORTS_BASE_CALLING: bool,
    const STORES_MOCK_DATA: bool,
> {
    _phantom_return_value: PhantomData<TReturnValue>,
    fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>,
    owner: &'rs TOwner,
    fn_callback_tuner: FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, STORES_MOCK_DATA>,
}

impl<
    'rs,
    TMock,
    TOwner,
    TArgRefsTuple: Copy,
    TReturnValue,
    const SUPPORTS_BASE_CALLING: bool,
    const STORES_MOCK_DATA: bool,
>
    FnTuner<
        'rs,
        TMock,
        TOwner,
        TArgRefsTuple,
        TReturnValue,
        SUPPORTS_BASE_CALLING,
        STORES_MOCK_DATA,
    >
{
    pub(crate) fn new(fn_config: Arc<RefCell<FnConfig<'rs, TMock>>>, owner: &'rs TOwner) -> Self {
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
    ) -> &FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, STORES_MOCK_DATA>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let dyn_return_value = transmute_lifetime!(DynReturnValue::new(return_value));
        let return_value_source = ReturnValueSource::SingleTime(dyn_return_value);
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_tuner;
    }

    pub fn returns_many<'a>(
        &self,
        return_values: impl IntoIterator<Item = TReturnValue>,
    ) -> &FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, STORES_MOCK_DATA>
    where
        TReturnValue: IReturnValue<'a> + 'a,
    {
        let return_value_sources = return_values
            .into_iter()
            .map(|x| transmute_lifetime!(DynReturnValue::new(x)))
            .map(ReturnValueSource::SingleTime);
        self.fn_config
            .borrow_mut()
            .add_return_value_sources(return_value_sources);
        return &self.fn_callback_tuner;
    }

    pub fn returns_always<'a>(
        &self,
        return_value: TReturnValue,
    ) -> &FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, STORES_MOCK_DATA>
    where
        TReturnValue: 'rs + 'a + IReturnValue<'a> + Clone,
    {
        let return_value_source = ReturnValueSource::Perpetual(Box::new(move || {
            transmute_lifetime!(DynReturnValue::new(return_value.clone()))
        }));
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_tuner;
    }

    pub fn returns_with<'a>(
        &self,
        f: impl Fn(TArgRefsTuple) -> TReturnValue + 'rs,
    ) -> &FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, STORES_MOCK_DATA> {
        let return_value_source = ReturnValueSource::Factory(Box::new(
            move |dyn_arg_refs_tuple: DynArgRefsTuple<'rs>| {
                let arg_refs_tuple: TArgRefsTuple = dyn_arg_refs_tuple.downcast_into::<TArgRefsTuple>();
                let result = f(arg_refs_tuple);
                return transmute_lifetime!(DynReturnValue::new(result));
            },
        ));
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_callback_tuner;
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple: Copy, const SUPPORTS_BASE_CALLING: bool>
    FnTuner<'rs, TMock, TOwner, TArgRefsTuple, (), SUPPORTS_BASE_CALLING, false>
{
    pub fn does(&self, mut callback: impl FnMut(TArgRefsTuple) + 'static) -> &'rs TOwner {
        let callback_with_mock =
            move |_mock: &TMock, arg_refs_tuple: TArgRefsTuple| callback(arg_refs_tuple);
        self.fn_config.borrow_mut().set_callback(callback_with_mock);
        return self.owner;
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple: Copy, const SUPPORTS_BASE_CALLING: bool>
    FnTuner<'rs, TMock, TOwner, TArgRefsTuple, (), SUPPORTS_BASE_CALLING, true>
{
    pub fn does(&self, callback: impl FnMut(&TMock, TArgRefsTuple) + 'static) -> &'rs TOwner {
        self.fn_config.borrow_mut().set_callback(callback);
        return self.owner;
    }
}

impl<'rs, TMock, TOwner, TArgRefsTuple: Copy, TReturnValue, const STORES_MOCK_DATA: bool>
    FnTuner<'rs, TMock, TOwner, TArgRefsTuple, TReturnValue, true, STORES_MOCK_DATA>
{
    pub fn call_base(
        &self,
    ) -> &FnReturnCallbackTuner<'rs, TMock, TOwner, TArgRefsTuple, STORES_MOCK_DATA> {
        self.fn_config.borrow_mut().set_call_base();
        return &self.fn_callback_tuner;
    }
}
