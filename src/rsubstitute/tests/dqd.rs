#[cfg(not(test))]
fn f() {}
#[cfg(test)]
use f::f;
#[cfg(test)]
#[allow(mismatched_lifetime_syntaxes)]
mod f {
    use super::*;
    use rsubstitute::for_generated::*;
    fn base_f() {}
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Clone)]
    pub struct f_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for f_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Debug, IArgsFormatter)]
    pub struct f_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgsChecker<f_Call<'__rsubstitute_arg_field_lifetime>>
        for f_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: f_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            vec![]
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct fBaseCaller {}
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<f_Call<'__rsubstitute_arg_field_lifetime>, ()> for fBaseCaller
    {
        fn call_base(&self, call: f_Call<'__rsubstitute_arg_field_lifetime>) {
            return base_f();
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct fMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        base_caller: Arc<RefCell<fBaseCaller>>,
        f_data: FnData<
            f_Call<'__rsubstitute_arg_field_lifetime>,
            f_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            fBaseCaller,
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct fMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<fMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct fMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<fMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct fMock {
        pub setup: fMockSetup<'static>,
        pub received: fMockReceived<'static>,
        data: Arc<fMockData<'static>>,
    }
    unsafe impl Send for fMock {}
    unsafe impl Sync for fMock {}
    impl<'__rsubstitute_arg_field_lifetime> Default for fMock {
        fn default() -> Self {
            let data = Arc::new(fMockData {
                _phantom_lifetime: PhantomData,
                base_caller: Arc::new(RefCell::new(fBaseCaller {})),
                f_data: FnData::new("f", &SERVICES),
            });
            return fMock {
                setup: fMockSetup { data: data.clone() },
                received: fMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> fMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            f_Call<'__rsubstitute_arg_field_lifetime>,
            f_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
            fBaseCaller,
        > {
            let f_args_checker = f_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.f_data.add_config(f_args_checker);
            let shared_fn_config =
                SharedFnConfig::new(fn_config, self, Some(self.data.base_caller.clone()));
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> fMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let f_args_checker = f_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data.f_data.verify_received(f_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rsubstitute_arg_field_lifetime>() -> SharedFnConfig<
        'static,
        f_Call<'static>,
        f_ArgsChecker<'static>,
        (),
        fMockSetup<'static>,
        fBaseCaller,
    > {
        let mock = get_global_mock::<fMock>();
        mock.data.f_data.reset();
        return mock.setup.setup();
    }
    pub fn received<'__rsubstitute_arg_field_lifetime>(
        times: Times,
    ) -> &'static fMockReceived<'static> {
        return get_global_mock::<fMock>().received.received(times);
    }
    pub fn f<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>() {
        let call = unsafe {
            f_Call {
                _phantom_lifetime: PhantomData,
            }
        };
        get_global_mock::<fMock>().data.f_data.handle_base(call);
    }
}
