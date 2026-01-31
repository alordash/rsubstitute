#[cfg(not(test))]
fn brпr() {}
#[cfg(test)]
use brпr::brпr;
#[cfg(test)]
#[allow(mismatched_lifetime_syntaxes)]
mod brпr {
    use super::*;
    use rsubstitute::for_generated::*;
    fn base_brпr() {}
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Clone)]
    pub struct brпr_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for brпr_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Debug, IArgsFormatter)]
    pub struct brпr_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<brпr_Call<'__rsubstitute_arg_field_lifetime>>
        for brпr_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(
            &self, call: brпr_Call<'__rsubstitute_arg_field_lifetime>
        ) -> Vec<ArgCheckResult> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct brпrMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        brпr_data: FnData<
            brпrMock,
            brпr_Call<'__rsubstitute_arg_field_lifetime>,
            brпr_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct brпrMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<brпrMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct brпrMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<brпrMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct brпrMock {
        pub setup: brпrMockSetup<'static>,
        pub received: brпrMockReceived<'static>,
        data: Arc<brпrMockData<'static>>,
    }
    unsafe impl Send for brпrMock {}
    unsafe impl Sync for brпrMock {}
    impl<'__rsubstitute_arg_field_lifetime> Default for brпrMock {
        fn default() -> Self {
            let data = Arc::new(brпrMockData {
                _phantom_lifetime: PhantomData,
                brпr_data: FnData::new("brпr", &SERVICES),
            });
            return brпrMock {
                setup: brпrMockSetup { data: data.clone() },
                received: brпrMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> brпrMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            brпrMock,
            brпr_Call<'__rsubstitute_arg_field_lifetime>,
            brпr_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
        > {
            let brпr_args_checker = brпr_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.brпr_data.add_config(brпr_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> brпrMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let brпr_args_checker = brпr_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .brпr_data
                .verify_received(brпr_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rsubstitute_arg_field_lifetime>() -> SharedFnConfig<
        'static,
        brпrMock,
        brпr_Call<'static>,
        brпr_ArgsChecker<'static>,
        (),
        brпrMockSetup<'static>,
    > {
        let mock = get_global_mock::<brпrMock>();
        mock.data.brпr_data.reset();
        return mock.setup.setup();
    }
    pub fn received<'__rsubstitute_arg_field_lifetime>(
        times: Times,
    ) -> &'static brпrMockReceived<'static> {
        return get_global_mock::<brпrMock>().received.received(times);
    }
    pub fn brпr<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>() {
        let call = unsafe {
            brпr_Call {
                _phantom_lifetime: PhantomData,
            }
        };
        get_global_mock::<brпrMock>()
            .data
            .brпr_data
            .handle_base(call);
    }
}
