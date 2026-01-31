#[cfg(not(test))]
fn ff() {}
#[cfg(test)]
use ff::ff;
#[cfg(test)]
#[allow(mismatched_lifetime_syntaxes)]
mod ff {
    use super::*;
    use rsubstitute::for_generated::*;
    fn base_ff() {}
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Clone)]
    pub struct ff_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for ff_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Debug, IArgsFormatter)]
    pub struct ff_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgsChecker<ff_Call<'__rsubstitute_arg_field_lifetime>>
        for ff_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: ff_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct ffMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        ff_data: FnData<
            ffMock<'__rsubstitute_arg_field_lifetime>,
            ff_Call<'__rsubstitute_arg_field_lifetime>,
            ff_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct ffMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<ffMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct ffMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<ffMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct ffMock {
        pub setup: ffMockSetup<'static>,
        pub received: ffMockReceived<'static>,
        data: Arc<ffMockData<'static>>,
    }
    unsafe impl Send for ffMock {}
    unsafe impl Sync for ffMock {}
    impl<'__rsubstitute_arg_field_lifetime> Default for ffMock {
        fn default() -> Self {
            let data = Arc::new(ffMockData {
                _phantom_lifetime: PhantomData,
                ff_data: FnData::new("ff", &SERVICES),
            });
            return ffMock {
                setup: ffMockSetup { data: data.clone() },
                received: ffMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> ffMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            ffMock,
            ff_Call<'__rsubstitute_arg_field_lifetime>,
            ff_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
        > {
            let ff_args_checker = ff_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.ff_data.add_config(ff_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> ffMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let ff_args_checker = ff_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data.ff_data.verify_received(ff_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rsubstitute_arg_field_lifetime>() -> SharedFnConfig<
        'static,
        ffMock,
        ff_Call<'static>,
        ff_ArgsChecker<'static>,
        (),
        ffMockSetup<'static>,
    > {
        let mock = get_global_mock::<ffMock>();
        mock.data.ff_data.reset();
        return mock.setup.setup();
    }
    pub fn received<'__rsubstitute_arg_field_lifetime>(
        times: Times,
    ) -> &'static ffMockReceived<'static> {
        return get_global_mock::<ffMock>().received.received(times);
    }
    pub fn ff<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>() {
        let call = unsafe {
            ff_Call {
                _phantom_lifetime: PhantomData,
            }
        };
        get_global_mock::<ffMock>().data.ff_data.handle_base(call);
    }
}
