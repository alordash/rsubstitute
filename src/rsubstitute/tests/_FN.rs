#[cfg(not(test))]
fn gffw() {}
#[cfg(test)]
use gffw::gffw;
#[cfg(test)]
#[allow(mismatched_lifetime_syntaxes)]
mod gffw {
    use super::*;
    use rsubstitute::for_generated::*;
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Clone)]
    pub struct gffw_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for gffw_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Debug, IArgsFormatter)]
    pub struct gffw_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<gffw_Call<'__rsubstitute_arg_field_lifetime>>
        for gffw_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: gffw_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            vec![]
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<gffw_Call<'__rsubstitute_arg_field_lifetime>, ()> for gffwMock
    {
        fn call_base(&self, call: gffw_Call<'__rsubstitute_arg_field_lifetime>) {
            let gffw_Call { .. } = call;
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct gffwMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        gffw_data: FnData<
            gffwMock,
            gffw_Call<'__rsubstitute_arg_field_lifetime>,
            gffw_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct gffwMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<gffwMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct gffwMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<gffwMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct gffwMock {
        pub setup: gffwMockSetup<'static>,
        pub received: gffwMockReceived<'static>,
        data: Arc<gffwMockData<'static>>,
    }
    unsafe impl Send for gffwMock {}
    unsafe impl Sync for gffwMock {}
    impl<'__rsubstitute_arg_field_lifetime> Default for gffwMock {
        fn default() -> Self {
            let data = Arc::new(gffwMockData {
                _phantom_lifetime: PhantomData,
                gffw_data: FnData::new("gffw", &SERVICES),
            });
            return gffwMock {
                setup: gffwMockSetup { data: data.clone() },
                received: gffwMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> gffwMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            gffwMock,
            gffw_Call<'__rsubstitute_arg_field_lifetime>,
            gffw_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
        > {
            let gffw_args_checker = gffw_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.gffw_data.add_config(gffw_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> gffwMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let gffw_args_checker = gffw_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .gffw_data
                .verify_received(gffw_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rsubstitute_arg_field_lifetime>() -> SharedFnConfig<
        'static,
        gffwMock,
        gffw_Call<'static>,
        gffw_ArgsChecker<'static>,
        (),
        gffwMockSetup<'static>,
    > {
        let mock = get_global_mock::<gffwMock>();
        mock.data.gffw_data.reset();
        return mock.setup.setup();
    }
    pub fn received<'__rsubstitute_arg_field_lifetime>(
        times: Times,
    ) -> &'static gffwMockReceived<'static> {
        return get_global_mock::<gffwMock>().received.received(times);
    }
    pub fn gffw<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>() {
        let call = unsafe {
            gffw_Call {
                _phantom_lifetime: PhantomData,
            }
        };
        let mock = get_global_mock::<gffwMock>();
        mock.data.gffw_data.handle_base(&mock, call);
    }
}
