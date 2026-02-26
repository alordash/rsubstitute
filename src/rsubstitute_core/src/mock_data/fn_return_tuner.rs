use crate::fn_parameters::*;
use crate::mock_data::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct FnReturnTuner<'rs, TOwner, TReturnValue> {
    _phantom_return_value: PhantomData<TReturnValue>,
    fn_config: Arc<RefCell<FnConfig<'rs>>>,
    fn_return_callback_tuner: FnReturnCallbackTuner<'rs, TOwner>,
}

impl<'rs, TOwner, TReturnValue> FnReturnTuner<'rs, TOwner, TReturnValue> {
    pub(crate) fn new(fn_config: Arc<RefCell<FnConfig<'rs>>>, owner: &'rs TOwner) -> Self {
        Self {
            _phantom_return_value: PhantomData,
            fn_config: fn_config.clone(),
            fn_return_callback_tuner: FnReturnCallbackTuner::new(fn_config, owner),
        }
    }

    pub fn many<'a>(
        &self,
        return_values: impl IntoIterator<Item = TReturnValue>,
    ) -> &FnReturnCallbackTuner<'rs, TOwner>
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
        return &self.fn_return_callback_tuner;
    }

    pub fn always<'a>(&self, return_value: TReturnValue) -> &FnReturnCallbackTuner<'rs, TOwner>
    where
        TReturnValue: 'rs + 'a + IReturnValue<'a> + Clone,
    {
        let return_value_source = ReturnValueSource::Perpetual(Box::new(move || unsafe {
            std::mem::transmute(DynReturnValue::new(return_value.clone()))
        }));
        self.fn_config
            .borrow_mut()
            .add_return_value_source(return_value_source);
        return &self.fn_return_callback_tuner;
    }
}
