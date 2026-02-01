#[cfg(not(test))]
fn accehtpt_rw(r: &i32) {}
#[cfg(test)]
use accehtpt_rw::accehtpt_rw;
#[cfg(test)]
#[allow(mismatched_lifetime_syntaxes)]
mod accehtpt_rw {
    use super::*;
    use rsubstitute::for_generated::*;
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Clone)]
    pub struct accehtpt_rw_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        r: &'__rsubstitute_arg_field_lifetime i32,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for accehtpt_rw_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new("r", self.r.clone())]
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Debug, IArgsFormatter)]
    pub struct accehtpt_rw_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        r: Arg<&'__rsubstitute_arg_field_lifetime i32>,
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<accehtpt_rw_Call<'__rsubstitute_arg_field_lifetime>>
        for accehtpt_rw_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(
            &self,
            call: accehtpt_rw_Call<'__rsubstitute_arg_field_lifetime>,
        ) -> Vec<ArgCheckResult> {
            vec![self.r.check_ref("r", call.r)]
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<accehtpt_rw_Call<'__rsubstitute_arg_field_lifetime>, ()> for accehtpt_rwMock
    {
        fn call_base(&self, call: accehtpt_rw_Call<'__rsubstitute_arg_field_lifetime>) {
            let accehtpt_rw_Call { r, .. } = call;
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct accehtpt_rwMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        accehtpt_rw_data: FnData<
            accehtpt_rwMock,
            accehtpt_rw_Call<'__rsubstitute_arg_field_lifetime>,
            accehtpt_rw_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct accehtpt_rwMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<accehtpt_rwMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct accehtpt_rwMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<accehtpt_rwMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct accehtpt_rwMock {
        pub setup: accehtpt_rwMockSetup<'static>,
        pub received: accehtpt_rwMockReceived<'static>,
        data: Arc<accehtpt_rwMockData<'static>>,
    }
    unsafe impl Send for accehtpt_rwMock {}
    unsafe impl Sync for accehtpt_rwMock {}
    impl<'__rsubstitute_arg_field_lifetime> Default for accehtpt_rwMock {
        fn default() -> Self {
            let data = Arc::new(accehtpt_rwMockData {
                _phantom_lifetime: PhantomData,
                accehtpt_rw_data: FnData::new("accehtpt_rw", &SERVICES),
            });
            return accehtpt_rwMock {
                setup: accehtpt_rwMockSetup { data: data.clone() },
                received: accehtpt_rwMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> accehtpt_rwMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
            r: impl Into<Arg<&'__rsubstitute_arg_field_lifetime i32>>,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            accehtpt_rwMock,
            accehtpt_rw_Call<'__rsubstitute_arg_field_lifetime>,
            accehtpt_rw_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
        > {
            let accehtpt_rw_args_checker = accehtpt_rw_ArgsChecker {
                _phantom_lifetime: PhantomData,
                r: r.into(),
            };
            let fn_config = self
                .data
                .accehtpt_rw_data
                .add_config(accehtpt_rw_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> accehtpt_rwMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            r: impl Into<Arg<&'__rsubstitute_arg_field_lifetime i32>>,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let accehtpt_rw_args_checker = accehtpt_rw_ArgsChecker {
                _phantom_lifetime: PhantomData,
                r: r.into(),
            };
            self.data
                .accehtpt_rw_data
                .verify_received(accehtpt_rw_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rsubstitute_arg_field_lifetime>(
        r: impl Into<Arg<&'static i32>>,
    ) -> SharedFnConfig<
        'static,
        accehtpt_rwMock,
        accehtpt_rw_Call<'static>,
        accehtpt_rw_ArgsChecker<'static>,
        (),
        accehtpt_rwMockSetup<'static>,
    > {
        let mock = get_global_mock::<accehtpt_rwMock>();
        mock.data.accehtpt_rw_data.reset();
        return mock.setup.setup(r);
    }
    pub fn received<'__rsubstitute_arg_field_lifetime>(
        r: impl Into<Arg<&'static i32>>,
        times: Times,
    ) -> &'static accehtpt_rwMockReceived<'static> {
        return get_global_mock::<accehtpt_rwMock>()
            .received
            .received(r, times);
    }
    pub fn accehtpt_rw<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>(
        r: &'__rsubstitute_arg_anonymous i32,
    ) {
        let call = unsafe {
            accehtpt_rw_Call {
                _phantom_lifetime: PhantomData,
                r: std::mem::transmute(r),
            }
        };
        let mock = get_global_mock::<accehtpt_rwMock>();
        mock.data.accehtpt_rw_data.handle_base(&mock, call);
    }
}
