#[cfg(not(test))]
fn fer() {}
#[cfg(test)]
use fer::fer;
#[cfg(test)]
#[allow(mismatched_lifetime_syntaxes)]
mod fer {
    use super::*;
    use rsubstitute::for_generated::*;
    fn base_fer() {}
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Clone)]
    pub struct fer_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for fer_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Debug, IArgsFormatter)]
    pub struct fer_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<fer_Call<'__rsubstitute_arg_field_lifetime>>
        for fer_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: fer_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            vec![]
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct ferBaseCaller {}
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<fer_Call<'__rsubstitute_arg_field_lifetime>, ()> for ferBaseCaller
    {
        fn call_base(&self, call: fer_Call<'__rsubstitute_arg_field_lifetime>) {
            return base_fer();
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct ferMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        fer_data: FnData<
            ferMock<'__rsubstitute_arg_field_lifetime>,
            fer_Call<'__rsubstitute_arg_field_lifetime>,
            fer_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            ferBaseCaller,
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct ferMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<ferMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct ferMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<ferMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct ferMock {
        pub setup: ferMockSetup<'static>,
        pub received: ferMockReceived<'static>,
        data: Arc<ferMockData<'static>>,
    }
    unsafe impl Send for ferMock {}
    unsafe impl Sync for ferMock {}
    impl<'__rsubstitute_arg_field_lifetime> Default for ferMock {
        fn default() -> Self {
            let data = Arc::new(ferMockData {
                _phantom_lifetime: PhantomData,
                base_caller: Arc::new(RefCell::new(ferBaseCaller {})),
                fer_data: FnData::new("fer", &SERVICES),
            });
            return ferMock {
                setup: ferMockSetup { data: data.clone() },
                received: ferMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> ferMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            fer_Call<'__rsubstitute_arg_field_lifetime>,
            fer_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
            ferBaseCaller,
        > {
            let fer_args_checker = fer_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.fer_data.add_config(fer_args_checker);
            let shared_fn_config =
                SharedFnConfig::new(fn_config, self, Some(self.data.base_caller.clone()));
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> ferMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let fer_args_checker = fer_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data.fer_data.verify_received(fer_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rsubstitute_arg_field_lifetime>() -> SharedFnConfig<
        'static,
        fer_Call<'static>,
        fer_ArgsChecker<'static>,
        (),
        ferMockSetup<'static>,
        ferBaseCaller,
    > {
        let mock = get_global_mock::<ferMock>();
        mock.data.fer_data.reset();
        return mock.setup.setup();
    }
    pub fn received<'__rsubstitute_arg_field_lifetime>(
        times: Times,
    ) -> &'static ferMockReceived<'static> {
        return get_global_mock::<ferMock>().received.received(times);
    }
    pub fn fer<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>() {
        let call = unsafe {
            fer_Call {
                _phantom_lifetime: PhantomData,
            }
        };
        get_global_mock::<ferMock>().data.fer_data.handle_base(call);
    }
}
