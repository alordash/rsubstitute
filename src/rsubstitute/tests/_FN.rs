#[cfg(not(test))]
fn grrg() {}
#[cfg(test)]
use grrg::grrg;
#[cfg(test)]
#[allow(mismatched_lifetime_syntaxes)]
mod grrg {
    use super::*;
    use rsubstitute::for_generated::*;
    fn base_grrg() {}
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Clone)]
    pub struct grrg_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for grrg_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Debug, IArgsFormatter)]
    pub struct grrg_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<grrg_Call<'__rsubstitute_arg_field_lifetime>>
        for grrg_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: grrg_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct grrgMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        grrg_data: FnData<
            grrgMock<'__rsubstitute_arg_field_lifetime>,
            grrg_Call<'__rsubstitute_arg_field_lifetime>,
            grrg_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct grrgMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<grrgMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct grrgMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<grrgMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct grrgMock {
        pub setup: grrgMockSetup<'static>,
        pub received: grrgMockReceived<'static>,
        data: Arc<grrgMockData<'static>>,
    }
    unsafe impl Send for grrgMock {}
    unsafe impl Sync for grrgMock {}
    impl<'__rsubstitute_arg_field_lifetime> Default for grrgMock {
        fn default() -> Self {
            let data = Arc::new(grrgMockData {
                _phantom_lifetime: PhantomData,
                grrg_data: FnData::new("grrg", &SERVICES),
            });
            return grrgMock {
                setup: grrgMockSetup { data: data.clone() },
                received: grrgMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> grrgMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            grrgMock,
            grrg_Call<'__rsubstitute_arg_field_lifetime>,
            grrg_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
        > {
            let grrg_args_checker = grrg_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.grrg_data.add_config(grrg_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> grrgMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let grrg_args_checker = grrg_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .grrg_data
                .verify_received(grrg_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rsubstitute_arg_field_lifetime>() -> SharedFnConfig<
        'static,
        grrgMock,
        grrg_Call<'static>,
        grrg_ArgsChecker<'static>,
        (),
        grrgMockSetup<'static>,
    > {
        let mock = get_global_mock::<grrgMock>();
        mock.data.grrg_data.reset();
        return mock.setup.setup();
    }
    pub fn received<'__rsubstitute_arg_field_lifetime>(
        times: Times,
    ) -> &'static grrgMockReceived<'static> {
        return get_global_mock::<grrgMock>().received.received(times);
    }
    pub fn grrg<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>() {
        let call = unsafe {
            grrg_Call {
                _phantom_lifetime: PhantomData,
            }
        };
        get_global_mock::<grrgMock>()
            .data
            .grrg_data
            .handle_base(call);
    }
}
